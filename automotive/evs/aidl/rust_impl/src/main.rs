//
// Copyright (C) 2024 The Android Open Source Project
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

mod default_evs_hal;

use crate::default_evs_hal::DefaultEvsHal;

use android_hardware_automotive_evs::aidl::android::hardware::automotive::evs::IEvsEnumerator::BnEvsEnumerator;

use log::info;

fn main() {
    binder::ProcessState::start_thread_pool();

    let service = DefaultEvsHal {};

    // Register HAL implementation as rust/0 instance.
    let service_name = "android.hardware.automotive.evs.IEvsEnumerator/rust/0";
    let service_binder = BnEvsEnumerator::new_binder(service, binder::BinderFeatures::default());

    binder::add_service(service_name, service_binder.as_binder())
        .expect(format!("Failed to register {}.", service_name).as_str());
    info!("EVS Hardware Enumerator is ready");

    binder::ProcessState::join_thread_pool();

    // In normal operation, we don't expect the thread pool to exit.
    info!("EVS Hardware Enumerator is shutting down");
}
