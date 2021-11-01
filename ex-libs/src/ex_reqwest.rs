// 请求样例
pub async fn r_example(url: &str) -> Result<(), reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    println!("body {}", body);
    Ok(())
}

// 发起GET请求
pub async fn r_get() -> Result<(), reqwest::Error> {
    let rsp = reqwest::get("https://rust-lang.org").await?;

    // 返回值的header
    let rsp_headers = rsp.headers();
    println!("rsp_headers: {:?}", rsp_headers);
    println!("cookies: {:?}", rsp_headers.get("set-cookie"));

    let status = rsp.status();
    println!("status: {}", status);
    // 判断状态码是否成功类型状态码：200-299.
    assert!(status.is_success());
    // 判断状态码是否是OK
    assert_eq!(status, reqwest::StatusCode::OK);

    // 状态码转换为数值：200
    let status_code = status.as_u16();
    println!("status_code: {}", status_code);
    assert_eq!(status_code, 200);

    // 获取返回的body数据，body获取需要await操作
    let body = rsp.text().await?;
    println!("body: {}", body);

    Ok(())
}

// reqwest默认只导出了get请求，即可以：reqwest::get()来请求，其他是需要创建Client来调用
pub async fn r_client() -> Result<(), reqwest::Error> {
    // 创建简单的client
    let client_1 = reqwest::Client::new();
    // client发送get,并带有query参数，设置header和timeout
    let rsp = client_1
        .get("https://rust-lang.org")
        .query(&[("key1", "value1"), ("key2", "value2")]) // 添加query
        .header("X-MY-HEADER", "client_1") // 设置header
        .timeout(std::time::Duration::from_secs(10)) // 设置timeout
        .send()
        .await?;
    println!("url: {}", rsp.url());
    assert!(rsp.content_length() > Some(1));

    // 创建全局的client
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::HOST, "rust-lang.org".parse().unwrap());
    headers.insert(
        reqwest::header::USER_AGENT, 
        reqwest::header::HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/95.0.4638.54 Safari/537.36")
    );
    let client_2 = reqwest::Client::builder().default_headers(headers).timeout(std::time::Duration::from_secs(10)).build()?;
    // post body
    let rsp_2 = client_2
        .post("http://httpbin.org/post")
        .body("some words")
        .send()
        .await?;
        assert!(rsp_2.status().is_success());
    
    // post form
    let params = [("foo", "bar"), ("baz", "quux")];
    let rsp_3 = client_2
        .post("http://httpbin.org/post")
        .form(&params)
        .send()
        .await?;
    assert!(rsp_3.status().is_success());

    // post json. features需要开启json
    let mut map = std::collections::HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");
    let rsp_4 = client_1
        .post("http://httpbin.org/post")
        .json(&map)
        .send()
        .await?;
    // 获取json返回结果
    let data_4 = rsp_4.json().await?;
    println!("{:?}", data_4);
    Ok(())
}

// 使用代理请求
pub async fn r_proxy() -> Result<(), reqwest::Error> {
    let proxy = reqwest::Proxy::http("https://secure.example")?;
    let client =reqwest::Client::builder().proxy(proxy).timeout(std::time::Duration::from_secs(10)).build()?;
    let rsp=  client.get("http://rust-lang.org").send().await?;
    println!("rsp status {}", rsp.status());
    Ok(())
}

// 不同的url对应不同的请求方法
pub async fn r_request() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    
    let rsp_1 = client.request(reqwest::Method::GET, "https://rust-lang.org").send().await?;
    println!("rsp_1 status {}", rsp_1.status());
    
    let rsp_2=  client.request(reqwest::Method::POST, "https://rust-lang.org").body("hahaha").send().await?;
    println!("rsp_2 status {}", rsp_2.status());
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn t_r_get() {
        let r = r_get().await;
        println!("{:?}", r);
    }

    #[tokio::test]
    async fn t_r_client() {
        let r = r_client().await;
        println!("{:?}", r);
    }

    #[tokio::test]
    async fn t_r_proxy() {
        let r = r_proxy().await;
        println!("{:?}", r);
    }

    #[tokio::test]
    async fn t_r_request() {
        let r = r_request().await;
        println!("{:?}", r);
    }
}
