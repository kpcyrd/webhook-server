// use std::sync::Arc;
// use std::sync::atomic::{Ordering, AtomicUsize};
use std::time::Duration;
use std::net::SocketAddr;
use std::collections::HashSet;

use tokio_core::reactor::Core;
use tokio_core::net::{TcpListener};
use futures::{Stream, Future};
use futures::future::{FutureResult, ok};

use tk_http::{Status};
use tk_http::server::buffered::{Request, BufferedDispatcher};
use tk_http::server::{Encoder, EncoderDone, Config, Proto, Error};
use tk_listen::ListenExt;

use serde_json;
use ipc_channel::ipc::IpcSender;

use sandbox;
use sandbox::ipc::Message;
use errors::Result;


#[derive(Debug, Serialize, Deserialize)]
pub struct JsonReply {
    status: String,
}

impl JsonReply {
    fn new<I: Into<String>>(status: I) -> JsonReply {
        JsonReply {
            status: status.into(),
        }
    }
}

fn get_useragent(req: &Request) -> Option<&str> {
    use std::str;

    req.headers().into_iter()
        .filter(|&&(ref key, _)| key.to_lowercase() == "user-agent")
        .flat_map(|&(_, ref value)| str::from_utf8(&value).ok())
        .next()
}

pub struct Server {
    addr: SocketAddr,
    hooks: HashSet<String>,
    tx: IpcSender<Message>,
}

impl Server {
    pub fn new(addr: SocketAddr, hooks: HashSet<String>, tx: IpcSender<Message>) -> Server {
        Server {
            addr,
            hooks,
            tx,
        }
    }

    #[inline]
    fn extract_hook<'a>(&self, path: &'a str) -> Option<&'a str> {
        let path = path.split("/");
        let hook_id = path.skip(2).next(); // https://example.com/narnia/<hook_id>

        // if valid hook
        match hook_id {
            Some(hook) if self.hooks.contains(hook) => Some(hook),
            _ => None,
        }
    }

    #[inline]
    fn service<S>(&self, _counter: usize, r: Request, e: Encoder<S>)
        -> FutureResult<EncoderDone<S>, Error>
    {
        info!("req: {} {:?} {:?} {:?} {:?}",
            r.peer_addr(),
            r.host().unwrap_or("-"),
            r.method(),
            r.path(),
            get_useragent(&r).unwrap_or("-"),
        );

        let success = match self.extract_hook(r.path()) {
            Some(hook) => {
                info!("queueing hook: {:?}", hook);
                self.tx.send(Message::new(hook)).unwrap();
                true
            },
            _ => false,
        };

        if success {
            self.write_response(e, Status::Ok, &JsonReply::new("queued"))
        } else {
            self.write_response(e, Status::NotFound, &JsonReply::new("not found"))
        }
    }

    fn write_response<S>(&self, mut e: Encoder<S>, status: Status, json: &JsonReply)
        -> FutureResult<EncoderDone<S>, Error>
    {
        let body = serde_json::to_string(json).expect("encode failed");

        e.status(status);
        e.add_length(body.as_bytes().len() as u64).unwrap();
        // e.format_header("Date", time::now_utc().rfc822()).unwrap();
        /*
        e.add_header("Server",
            concat!("narnia/", env!("CARGO_PKG_VERSION"))
        ).unwrap();
        */
        if e.done_headers().unwrap() {
            e.write_body(body.as_bytes());
        }
        ok(e.done())
    }

    pub fn start(&self) -> Result<()> {
        let mut lp = Core::new().unwrap();
        let listener = TcpListener::bind(&self.addr, &lp.handle())?;
        let cfg = Config::new().done();
        let h1 = lp.handle();

        sandbox::activate_stage2().expect("failed to activate stage2");

        let done = listener.incoming()
            .sleep_on_error(Duration::from_millis(100), &lp.handle())
            .map(move |(socket, addr)| {
                Proto::new(socket, &cfg,
                    BufferedDispatcher::new(addr, &h1, move || {
                        move |r, e| {
                            let val = 123;
                            self.service(val, r, e)
                        }
                    }),
                    &h1)
                .map_err(|e| { println!("Connection error: {}", e); })
            })
            .listen(1000);

        lp.run(done).unwrap();

        Ok(())
    }
}
