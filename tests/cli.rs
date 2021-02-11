use assert_cmd::prelude::*;
use indoc::indoc;
use predicates::prelude::*;
use std::process::Command;

fn get_command() -> Command {
    let mut cmd = Command::cargo_bin("ht").expect("binary should be present");
    cmd.env("HT_TEST_MODE", "1");
    cmd
}

#[test]
fn basic_post() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = get_command();
    cmd.arg("-v")
        .arg("--offline")
        .arg("--ignore-stdin")
        .arg("--pretty=format")
        .arg("post")
        .arg("httpbin.org/post")
        .arg("name=ali");

    cmd.assert().stdout(indoc! {r#"
        POST /post HTTP/1.1
        accept: application/json, */*
        accept-encoding: gzip, deflate
        connection: keep-alive
        content-length: 14
        content-type: application/json
        host: httpbin.org
        user-agent: ht/0.0.0 (test mode)

        {
            "name": "ali"
        }

    "#});

    Ok(())
}

#[test]
fn basic_get() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = get_command();
    cmd.arg("-v")
        .arg("--offline")
        .arg("--ignore-stdin")
        .arg("--pretty=format")
        .arg("get")
        .arg("httpbin.org/get");

    cmd.assert().stdout(indoc! {r#"
        GET /get HTTP/1.1
        accept: */*
        accept-encoding: gzip, deflate
        connection: keep-alive
        host: httpbin.org
        user-agent: ht/0.0.0 (test mode)

    "#});

    Ok(())
}

#[test]
fn basic_head() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = get_command();
    cmd.arg("-v")
        .arg("--offline")
        .arg("--ignore-stdin")
        .arg("--pretty=format")
        .arg("head")
        .arg("httpbin.org/get");

    cmd.assert().stdout(indoc! {r#"
        HEAD /get HTTP/1.1
        accept: */*
        accept-encoding: gzip, deflate
        connection: keep-alive
        host: httpbin.org
        user-agent: ht/0.0.0 (test mode)

    "#});

    Ok(())
}

#[test]
fn basic_options() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("ht")?;
    cmd.arg("-v")
        .arg("--ignore-stdin")
        .arg("--pretty=format")
        .arg("options")
        .arg("httpbin.org/json");

    // Verify that the response is ok and contains an 'allow' header.
    cmd.assert()
        .stdout(predicate::str::contains("HTTP/1.1 200 OK"));
    cmd.assert().stdout(predicate::str::contains("allow:"));

    Ok(())
}

#[test]
fn multiline_value() {
    let mut cmd = get_command();
    cmd.arg("-v")
        .arg("--offline")
        .arg("--ignore-stdin")
        .arg("--pretty=format")
        .arg("--form")
        .arg("post")
        .arg("httpbin.org/post")
        .arg("foo=bar\nbaz");

    cmd.assert().stdout(indoc! {r#"
        POST /post HTTP/1.1
        accept: */*
        accept-encoding: gzip, deflate
        connection: keep-alive
        content-length: 13
        content-type: application/x-www-form-urlencoded
        host: httpbin.org
        user-agent: ht/0.0.0 (test mode)

        foo=bar%0Abaz

    "#});
}
#[test]
fn https_self_signed_cert_error() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = get_command();
    cmd.arg("-v")
        .arg("--pretty=format")
        .arg("get")
        .arg("https://self-signed.badssl.com");

    cmd.assert().stdout(indoc! {r#"
        GET / HTTP/1.1
        accept: application/json, */*
        accept-encoding: gzip, deflate
        connection: keep-alive
        content-length: 0
        content-type: application/json
        host: self-signed.badssl.com
        user-agent: ht/0.0.0 (test mode)



    "#});

    cmd.assert().stderr(indoc! {r#"
    Error: error sending request for url (https://self-signed.badssl.com/): error trying to connect: invalid certificate: UnknownIssuer

    Caused by:
        0: error trying to connect: invalid certificate: UnknownIssuer
        1: invalid certificate: UnknownIssuer
    "#});

    Ok(())
}

#[test]
fn https_self_signed_cert_skip() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = get_command();
    cmd.arg("-v")
        .arg("--pretty=format")
        .arg("--verify=false")
        .arg("get")
        .arg("https://self-signed.badssl.com");

    cmd.assert().stdout(indoc! {r#"
        GET / HTTP/1.1
        accept: application/json, */*
        accept-encoding: gzip, deflate
        connection: keep-alive
        content-length: 0
        content-type: application/json
        host: self-signed.badssl.com
        user-agent: ht/0.0.0 (test mode)



    "#});

    cmd.assert().stderr(indoc! {r#"
    Error: error sending request for url (https://self-signed.badssl.com/): error trying to connect: invalid certificate: UnknownIssuer

    Caused by:
        0: error trying to connect: invalid certificate: UnknownIssuer
        1: invalid certificate: UnknownIssuer
    "#});

    Ok(())
}