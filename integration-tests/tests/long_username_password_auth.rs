use test_utils::*;
use tokio_socks::{
    tcp::socks5::{Socks5Listener, Socks5Stream},
    Result,
};

#[test]
#[cfg(feature = "tokio")]
fn connect_long_username_password() -> Result<()> {
    let runtime = runtime().lock().unwrap();
    let conn = runtime.block_on(Socks5Stream::connect_with_password(
        PROXY_ADDR, ECHO_SERVER_ADDR, "mylonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglogin",
                                                                    "longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglongpassword"))?;
    runtime.block_on(test_connect(conn))
}

#[test]
#[cfg(feature = "tokio")]
fn bind_long_username_password() -> Result<()> {
    let bind = {
        let runtime = runtime().lock().unwrap();
        runtime.block_on(Socks5Listener::bind_with_password(
            PROXY_ADDR,
            ECHO_SERVER_ADDR,
            "mylonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglogin",
            "longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglongpassword"
        ))
    }?;
    test_bind(bind)
}

#[test]
fn connect_with_socket_long_username_password() -> Result<()> {
    let runtime = runtime().lock().unwrap();
    let socket = runtime.block_on(connect_unix(UNIX_PROXY_ADDR))?;
    let conn = runtime.block_on(Socks5Stream::connect_with_password_and_socket(
        socket, ECHO_SERVER_ADDR, "mylonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglogin",
                                                                    "longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglongpassword"))?;
    runtime.block_on(test_connect(conn))
}

#[test]
fn bind_with_socket_long_username_password() -> Result<()> {
    let bind = {
        let runtime = runtime().lock().unwrap();
        let socket = runtime.block_on(connect_unix(UNIX_PROXY_ADDR))?;
        runtime.block_on(Socks5Listener::bind_with_password_and_socket(
            socket,
            ECHO_SERVER_ADDR,
            "mylonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglogin",
            "longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglongpassword"
        ))
    }?;
    test_bind(bind)
}