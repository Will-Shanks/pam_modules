extern crate pam;
extern crate rand;

use pam::constants::{PamFlag, PamResultCode};
use pam::module::{PamHandle, PamHooks};
use rand::Rng;
use std::collections::HashMap;
use std::ffi::CStr;
use std::{thread, time};

struct PamFake;
pam::pam_hooks!(PamFake);

impl PamHooks for PamFake {
    // This function performs the task of authenticating the user.
    fn sm_authenticate(_pamh: &mut PamHandle, args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        //TODO: collect args from pam config
        //min_sleep (u32) seconds
        //max_sleep (u32) seconds
        //success_chance (float) decimal
        let args: HashMap<&str, &str> = args
            .iter()
            .map(|s| {
                let mut parts = s.to_str().unwrap().splitn(2, "=");
                (parts.next().unwrap(), parts.next().unwrap_or(""))
            })
            .collect();

        let zero = "0";
        let one = "1";
        eprintln!("[DEBUG]: args: {:?}", args);
        let min_sleep = args.get("min_sleep").unwrap_or(&zero).parse().unwrap_or(0);
        let max_sleep = args
            .get("max_sleep")
            .unwrap_or(&min_sleep.to_string().as_str())
            .parse()
            .unwrap_or(min_sleep);
        let success_chance: f64 = args
            .get("success_chance")
            .unwrap_or(&one)
            .parse()
            .unwrap_or(1.);

        let mut rng = rand::thread_rng();
        let sleep_sec = min_sleep + (rng.gen::<u64>() % (max_sleep - min_sleep));
        let pass: bool = rng.gen::<f64>() < success_chance;

        eprintln!("[DEBUG]: sleep: {} pass: {}", sleep_sec, pass);

        thread::sleep(time::Duration::from_secs(sleep_sec));
        if pass {
            PamResultCode::PAM_SUCCESS
        } else {
            PamResultCode::PAM_AUTH_ERR
        }
    }

    fn sm_setcred(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        println!("set credentials");
        PamResultCode::PAM_SUCCESS
    }

    fn acct_mgmt(_pamh: &mut PamHandle, _args: Vec<&CStr>, _flags: PamFlag) -> PamResultCode {
        println!("account management");
        PamResultCode::PAM_SUCCESS
    }
}
