use bytes::Bytes;
use command_lines::{Result, TokioIo};
use http_body_util::Full;
use hyper::{Request, Response, body::Incoming, service::Service};
use std::{
  env::current_exe,
  net::SocketAddr,
  path::PathBuf,
  pin::Pin,
  sync::{Arc, Mutex},
};
use tokio::net::TcpListener;

#[derive(Clone, Debug)]
struct PathStore {
  curr_base_path: Arc<Mutex<PathBuf>>,
}

impl PathStore {
  fn new() -> Self {
    let mut curr_exec_path = match current_exe() {
      Ok(val) => val,
      Err(_) => {
        println!("error: 没有获取到执行程序当前的路径信息");
        PathBuf::new()
      }
    };

    #[cfg(target_os = "windows")]
    {
      if curr_exec_path.ends_with("command-lines.exe") {
        curr_exec_path.pop();
      }
    }

    #[cfg(not(target_os = "windows"))]
    {
      if curr_exec_path.ends_with("command-lines") {
        curr_exec_path.pop();
      }
    }

    PathStore {
      curr_base_path: Arc::new(Mutex::new(curr_exec_path)),
    }
  }

  // 获取当前路径下的文件
  fn get_files(&self) -> Option<Vec<String>> {
    let curr_path = if let Ok(val) = self.curr_base_path.lock() {
      val.clone()
    } else {
      return None;
    };
    if curr_path.is_dir() {
      let mut temp_vec = Vec::new();

      if let Ok(val) = std::fs::read_dir(curr_path) {
        val.for_each(|item| {
          if let Ok(item) = item {
            temp_vec.push(item.file_name().to_str().unwrap().to_string());
          }
        });
      }
      dbg!(&temp_vec);
      return Some(temp_vec);
    }

    None
  }

  fn make_response(&self) -> Result<Response<Full<Bytes>>> {
    // let res = self.curr_base_path.lock().expect("获取基路径失败");
    // let res = res.to_str().unwrap();
    // let res = self.get_files().expect("获取失败").join(",");
    let res = match self.get_files() {
      Some(val) => val.join(","),
      None => {
        panic!("获取文件失败")
      }
    };
    Ok(
      if let Ok(val) = Response::builder().body(Full::new(Bytes::from(res))) {
        val
      } else {
        Response::builder()
          .body(Full::new(Bytes::from("fuck tcl")))
          .unwrap()
      },
    )
  }
}

impl Service<Request<Incoming>> for PathStore {
  type Error = hyper::Error;
  type Response = Response<Full<Bytes>>;
  type Future = Pin<
    Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>> + Send>,
  >;

  fn call(&self, _req: Request<Incoming>) -> Self::Future {
    let res = self.make_response().unwrap();
    Box::pin(async { Ok(res) })
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  // let out_dir = std::env::var("OUT_DIR").expect("报错了");
  // println!("output: {}", out_dir);

  let socket_addr: SocketAddr = ([127, 0, 0, 1], 8083).into();
  let listener = TcpListener::bind(socket_addr).await?;
  println!("listener on http://{}", socket_addr);

  let path_store = PathStore::new();

  loop {
    let (stream, _) = listener.accept().await?;
    let io = TokioIo::new(stream);
    let temp_path_store = path_store.clone();

    tokio::task::spawn(async move {
      if let Err(error) = hyper::server::conn::http1::Builder::new()
        .serve_connection(io, temp_path_store)
        .await
      {
        println!("Failed to serve connection: {:?}", error);
      }
    });
  }
}
