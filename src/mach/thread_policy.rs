use libc::{pthread_set_qos_class_self_np, qos_class_t};

use mach2::thread_policy::*;
use mach2::kern_return::KERN_SUCCESS;
use mach2::mach_init::mach_thread_self;

const QOS_CLASS_USER_INTERACTIVE: qos_class_t = qos_class_t::QOS_CLASS_USER_INTERACTIVE;


pub fn force_p_cores() {
    unsafe {
        // Set QoS Class to User Interactive Hoping the mach kernel will allow performance cores usage
        let res = pthread_set_qos_class_self_np(QOS_CLASS_USER_INTERACTIVE, 0);
        if res != 0 {
            eprintln!("Error setting QoS");
        }

        // Used to hint to use the P core cluster, to prevent it from bouncing between
        // P and E cores on apple silicon
        let thread_port = mach2::mach_init::mach_thread_self();
        let mut policy = mach2::thread_policy::thread_affinity_policy_data_t {
            affinity_tag: 1
        };

        mach2::thread_policy::thread_policy_set(
            thread_port,
            mach2::thread_policy::THREAD_AFFINITY_POLICY,
            &mut policy as *mut _ as *mut i32,
            mach2::thread_policy::THREAD_AFFINITY_POLICY_COUNT,
        );
    }
}

#[allow(dead_code)]
/// Trying to pin on wacked Mach restricted kernel with SIP enabled
pub fn pin_thread_to_affinity(affinity_tag: i32) {
    unsafe {
        let thread = mach_thread_self();
        let policy = thread_affinity_policy_data_t { affinity_tag };
        let ret = thread_policy_set(
            thread,
            THREAD_AFFINITY_POLICY,
            &policy as *const _ as *mut i32,
            THREAD_AFFINITY_POLICY_COUNT,
        );
        if ret != KERN_SUCCESS {
            eprintln!("Warning: failed to set thread affinity (ret={})", ret);
        }
    }
}