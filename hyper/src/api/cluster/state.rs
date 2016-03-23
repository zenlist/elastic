//! http://www.elastic.co/guide/en/elasticsearch/reference/master/cluster-state.html

//Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn get_metric_index<'a>(client: &'a mut Client, base: &'a str,
                        metric: &'a str, index: &'a str) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 16 + 1 + metric.len() +
                                  index.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_cluster/state/");
    url_fmtd.push_str(metric);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get<'a>(client: &'a mut Client, base: &'a str) -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 15);
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_cluster/state");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_metric<'a>(client: &'a mut Client, base: &'a str, metric: &'a str)
 -> Result<Response>{
    let mut url_fmtd = String::with_capacity(base.len() + 16 + metric.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/_cluster/state/");
    url_fmtd.push_str(metric);
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
