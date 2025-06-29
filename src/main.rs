use bytes::Bytes;
use command_lines::{
  Result,
  node::{self, NodeName},
  tools::{TokioIo, make_response_parts},
};
use http_body_util::Full;
use hyper::{
  Request, Response, StatusCode,
  body::{Body, Incoming},
  header,
  service::Service,
};
use std::{
  env::current_exe,
  io::Read,
  net::SocketAddr,
  path::{Path, PathBuf},
  pin::Pin,
  sync::{Arc, Mutex},
};
use tokio::net::TcpListener;
use tracing::{debug, error, info, trace, warn};

#[derive(Clone, Debug)]
struct PathStore {
  base_path: Arc<Mutex<PathBuf>>,    // 可执行文件地址
  current_path: Arc<Mutex<PathBuf>>, // 当前路由所在地址
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
      base_path: Arc::new(Mutex::new(curr_exec_path.clone())),
      current_path: Arc::new(Mutex::new(curr_exec_path)),
    }
  }

  // 加载文件
  fn load_static_file(&self, file_name: &str) -> Option<Vec<u8>> {
    let curr_path = self.base_path.lock();
    if curr_path.is_err() {
      return None;
    }

    let mut curr_path = curr_path.unwrap().clone();
    curr_path.push("static");
    curr_path.push(file_name);

    if !curr_path.exists() {
      return None;
    }

    let Ok(mut file) = std::fs::OpenOptions::new().read(true).open(curr_path) else {
      return None;
    };

    let capacity = if let Ok(meta) = file.metadata() {
      meta.len()
    } else {
      1024
    };

    let mut content = Vec::<u8>::with_capacity(capacity as usize);
    let _ = file.read_to_end(&mut content);
    Some(content)
  }

  fn handle_static_file_response(
    &self,
    file_name: &str,
  ) -> Result<Response<Full<Bytes>>> {
    let content = self.load_static_file(file_name).ok_or("加载文件失败")?;
    let content_length = content.len();
    let mut response = Response::builder().body(Full::new(Bytes::from(content)))?;

    let ext = Path::new(file_name)
      .extension()
      .ok_or("找不到扩招名，报错了")?
      .to_str()
      .ok_or("获取扩展名失败")?;

    let mut temp_header = make_response_parts(ext)?;
    if temp_header.append(
      header::CONTENT_LENGTH,
      content_length.to_string().parse().unwrap(),
    ) {
      error!("写入content_length失败");
    }

    *response.headers_mut() = temp_header;
    Ok(response)
  }

  // 获取当前路径下的文件
  fn get_files(&self) -> Option<Vec<String>> {
    let curr_path: PathBuf = if let Ok(val) = self.current_path.lock() {
      val.clone()
    } else {
      return None;
    };
    info!("当前路径：{:?}", curr_path);
    if curr_path.is_dir() {
      let mut temp_vec = Vec::new();

      if let Ok(val) = std::fs::read_dir(curr_path) {
        val.for_each(|item| {
          if let Ok(item) = item {
            temp_vec.push(item.file_name().to_str().unwrap().to_string());
          }
        });
      }
      return Some(temp_vec);
    }

    None
  }

  fn handle_empty(&self) -> Result<String> {
    todo!()
  }

  // 将文件转成node结构数据
  fn to_nodes(&self) -> Result<String> {
    let file_list = self.get_files().ok_or("获取文件列表失败")?;
    // info!("files: {:?}", file_list);
    let mut container = node::Node::new_node(NodeName::Div);
    container.attr_insert(
      "style",
      "display: grid; grid-template-columns: 1fr 1fr 1fr;grid-row-gap: 3px;grid-column-gap: 10px;",
    )?;
    let mut base_file_path = self.current_path.lock().map(|item| item.clone()).unwrap();
    if !file_list.is_empty() {
      for file in file_list {
        base_file_path.push(&file);
        if base_file_path.is_file() {
          let mut node = node::Node::new_node(NodeName::A);
          let text_node = node::Node::new_text(&file);
          node.append_child(text_node)?;
          node.attr_insert("style", "color: red;")?;
          node.attr_insert("href", &format!("/{}", &file))?;
          container.append_child(node)?;
        } else if base_file_path.is_dir() {
          let mut node = node::Node::new_node(NodeName::A);
          let text_node = node::Node::new_text(&file);
          node.append_child(text_node)?;
          let _ = node.attr_insert("style", "color: orange;");
          let _ = node.attr_insert("href", &format!("/{}", &file));
          let _ = container.append_child(node);
        }
        base_file_path.pop();
      }
    } else {
      let empty_text = node::Node::new_text("文件夹内容为空");
      container.append_child(empty_text)?;
    }

    let mut html = node::Node::new_node(NodeName::Html);
    let mut head = node::Node::new_node(NodeName::Head);
    let mut title = node::Node::new_node(NodeName::Title);
    let mut meta = node::Node::new_node(NodeName::Meta);
    meta.attr_insert("charset", "UTF-8")?;
    title.append_child(node::Node::new_text("vans"))?;
    head.append_child(title)?;
    head.append_child(meta)?;
    html.append_child(head)?;

    let mut body = node::Node::new_node(NodeName::Body);
    let mut head_route = node::Node::new_node(NodeName::Div);
    let mut back_element = node::Node::new_node(NodeName::A);
    back_element.append_child(node::Node::new_text("返回上一级"))?;
    back_element.attr_insert("href", "/back")?;
    head_route.append_child(back_element)?;
    body.append_child(head_route)?;
    body.append_child(container)?;
    html.append_child(body)?;

    let result = html.to_html()?;
    Ok(result)
  }

  fn make_response(&self) -> Result<Response<Full<Bytes>>> {
    let html = self.to_nodes().unwrap();
    let response_length = html.len();
    Ok(
      if let Ok(mut val) = Response::builder()
        .header(header::CONTENT_LENGTH, response_length)
        .body(Full::new(Bytes::from(html)))
      {
        *val.headers_mut() = make_response_parts("html")?;
        val
      } else {
        Response::builder()
          .body(Full::new(Bytes::from("fuck tcl")))
          .unwrap()
      },
    )
  }

  // 处理路由重定向
  fn handle_redirect(&self, pathname: String) -> Result<Response<Full<Bytes>>> {
    // 更新当前路径
    let mut curr_path = self.current_path.lock().unwrap();
    curr_path.push(pathname);

    if curr_path.is_dir() {
      info!("是文件");
      return Ok(
        Response::builder()
          .status(StatusCode::FOUND)
          .header(header::LOCATION, "/")
          .body(Full::new(Bytes::from("nothing")))
          .unwrap(),
      );
    } else if curr_path.is_file() {
      // 需要处理浏览器
      let res = curr_path.display();
      let file_while_list = ["html", "css", "js", "png", "jpg"];
      let ext = curr_path
        .extension()
        .expect("获取后缀名失败")
        .to_str()
        .unwrap();
      println!("文件后缀：{:?}", ext);
      if file_while_list.contains(&ext) {
        // 读取文件返回
        let mut content = Vec::new();
        let mut file = std::fs::OpenOptions::new()
          .read(true)
          .open(&*curr_path)
          .unwrap();
        let count = file.read_to_end(&mut content).unwrap();
        info!("读取的内容长度: {:?}", count);
        curr_path.pop();
        return Ok(
          Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_LENGTH, count)
            .header(header::CONTENT_TYPE, mime::TEXT_HTML_UTF_8.to_string())
            .body(Full::new(Bytes::from(content)))
            .unwrap(),
        );
      }
    }
    info!("回到上一级");
    curr_path.pop();

    Ok(
      Response::builder()
        .body(Full::new(Bytes::from("")))
        .unwrap(),
    )
  }

  // 处理返回上一级逻辑
  fn handle_back(&self) -> Result<Response<Full<Bytes>>> {
    self.current_path.lock().unwrap().pop();

    let response = Response::builder()
      .status(StatusCode::FOUND)
      .header(header::LOCATION, "/")
      .body(Full::new(Bytes::new()))
      .unwrap();

    Ok(response)
  }
}

impl Service<Request<Incoming>> for PathStore {
  type Error = hyper::Error;
  type Response = Response<Full<Bytes>>;
  type Future = Pin<
    Box<dyn Future<Output = std::result::Result<Self::Response, Self::Error>> + Send>,
  >;

  fn call(&self, req: Request<Incoming>) -> Self::Future {
    let path = req.uri().path();

    info!("request path: {path}");

    match path {
      "/" => {
        let res = self.make_response().unwrap();
        return Box::pin(async { Ok(res) });
      }
      "/404" => {
        if let Ok(val) = self.handle_static_file_response("404.html") {
          return Box::pin(async { Ok(val) });
        }
      }
      "/favicon.ico" => {
        if let Ok(val) = self.handle_static_file_response("favicon.ico") {
          return Box::pin(async { Ok(val) });
        }
      }
      _ => {}
    }

    if path.starts_with("/") {
      let mut temp_path = path.to_string();
      temp_path.remove(0);
      if temp_path == "back" {
        // 返回上一级
        let res = self.handle_back().unwrap();
        return Box::pin(async { Ok(res) });
      }
      let files = self.get_files().expect("获取文件失败");
      if files.contains(&temp_path) {
        // 需要处理路由重定向
        let res = match self.handle_redirect(temp_path) {
          Ok(val) => val,
          Err(error) => {
            info!("报错提示: {:?}", error);
            panic!();
          }
        };
        return Box::pin(async { Ok(res) });
      }
    }

    let result = if let Ok(val) = self.handle_static_file_response("nothing.html") {
      val
    } else {
      self.make_response().unwrap()
    };
    Box::pin(async { Ok(result) })
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::TRACE)
    .init();

  let socket_addr: SocketAddr = ([127, 0, 0, 1], 8083).into();
  let listener = TcpListener::bind(socket_addr).await?;
  info!("listener on http://{}", socket_addr);

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
