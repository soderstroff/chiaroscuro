#[macro_use(seq)]
extern crate chiaro;
use chiaro::*;

fn main() {

    let endl = seq! {
        option(char(b'\r'));
        char(b'\n')
    };

    let http_version = seq! {
        string("HTTP/");
        version = take_while(is_http_version);
        ret(version)
    };

    let request_line =
        take_while(is_token).bind(
            move |method| { take_while(is_space).seq(take_while(is_not_space)).bind(
                move |uri| {
                    take_while(is_space).seq(&http_version).bind(
                        move |version| { ret(request(method, uri, version))}
                        )
                }
                )}
            );
    // let request_line = seq! {
    //     method = take_while(is_token);
    //     take_while(is_space);
    //     uri = take_while(is_not_space);
    //     take_while(is_space);
    //     version = http_version;
    //     ret(request(method, uri, version))
    // };

    // let message_header_line = seq! {
    //     take_while(is_horizontal_space);
    //     line = take_while(is_not_end_of_line);
    //     endl;
    //     ret(line)
    // };

    // let message_header = seq! {
    //     name = take_while(is_token);
    //     char(b':');
    //     lines = collect_many1(message_header_line);
    //     ret(header(name, lines))
    // };
    //
    // let message_header = take_while(is_token).bind(
    //     |name| { char(b':').seq(collect_many1(message_header_line)).bind(
    //         |lines| { ret(header(name, lines))}
    //         )}
    //     );
    //
    // let request = seq! {
    //     r = request_line;
    //     endl;
    //     h = many(message_header);
    //     endl;
    //     ret((r, h))
    // };
}

#[derive(Clone)]
struct Request<'a> {
    method:  &'a [u8],
    uri:     &'a [u8],
    version: &'a [u8],
}

fn request<'a>(m: &'a [u8], u: &'a [u8], v: &'a [u8]) -> Request<'a> {
    Request { method: m , uri: u, version: v }
}

#[derive(Debug, Clone)]
struct Header<'a> {
    name:  &'a [u8],
    value: Vec<&'a [u8]>,
}

fn header<'a>(n: &'a [u8], v: Vec<&'a [u8]>) -> Header<'a> {
    Header { name: n, value: v }
}


fn is_token(c: u8) -> bool {
    c < 128 && c > 31 && b"()<>@,;:\\\"/[]?={} \t".iter().position(|&i| i == c).is_none()
}

fn is_horizontal_space(c: u8) -> bool { c == b' ' || c == b'\t' }
fn is_space(c: u8)            -> bool { c == b' ' }
fn is_not_space(c: u8)        -> bool { c != b' ' }
fn is_not_end_of_line(c: u8)      -> bool { !(c == b'\r' || c == b'\n') }
fn is_http_version(c: u8)     -> bool { c >= b'0' && c <= b'9' || c == b'.' }
