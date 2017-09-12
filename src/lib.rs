extern crate hyper;
extern crate websocket;

macro_rule! api_concat {
    ($e:expr) => (concat!("https://api.spotify.com/v1", $e))
}

macro_rules! request {
    ($self_:ident, $method:ident($body:expr), $url:expr, $($rest:tt)*) => {{
		let path = format!(api_concat!($url), $($rest)*);
		try!($self_.request(&path, || $self_.client.$method(&path).body(&$body)))
	}};
	($self_:ident, $method:ident, $url:expr, $($rest:tt)*) => {{
		let path = format!(api_concat!($url), $($rest)*);
		try!($self_.request(&path, || $self_.client.$method(&path)))
	}};
	($self_:ident, $method:ident($body:expr), $url:expr) => {{
		let path = api_concat!($url);
		try!($self_.request(path, || $self_.client.$method(path).body(&$body)))
	}};
	($self_:ident, $method:ident, $url:expr) => {{
		let path = api_concat!($url);
		try!($self_.request(path, || $self_.client.$method(path)))
	}};
}
