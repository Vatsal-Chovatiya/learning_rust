enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;


fn route(ip_kind: IpAddrKind) {}

// You can call it with either variant
route(IpAddrKind::V4);
route(IpAddrKind::V6);