//! FORTRAN Hello World on COR24 — minimal Yew/WASM live demo.
//!
//! The page embeds a pre-built `.lgo` produced upstream by the
//! SNOBOL4-based FTI-0 compiler in `sw-cor24-fortran`. On mount we load
//! the `.lgo` into a `cor24_emulator::EmulatorCore` and run it; UART
//! output is rendered on the right.
//!
//! Until dcftn ships a real `examples/hello.lgo`, the file is committed
//! as a 0-byte placeholder; in that mode the page shows a "pending"
//! notice instead of attempting to run.

use std::cell::RefCell;
use std::rc::Rc;

use cor24_emulator::{EmulatorCore, StopReason};
use yew::prelude::*;

const HELLO_F_SOURCE: &str = include_str!("../examples/hello.f");
const HELLO_LGO_BYTES: &[u8] = include_bytes!("../examples/hello.lgo");

const MAX_INSTRUCTIONS_PER_TICK: u64 = 50_000;
const TICK_INTERVAL_MS: u32 = 16;

#[function_component(App)]
fn app() -> Html {
    let uart_output = use_state(String::new);
    let status_msg = use_state(|| String::from("Loading..."));
    let halted = use_state(|| false);
    let running = use_state(|| false);
    let instr_count = use_state(|| 0u64);

    let emu: Rc<RefCell<EmulatorCore>> = use_mut_ref(EmulatorCore::new);
    let interval_handle = use_mut_ref(|| None::<gloo_timers::callback::Interval>);

    let lgo_text = match std::str::from_utf8(HELLO_LGO_BYTES) {
        Ok(s) => Some(s),
        Err(_) => None,
    };
    let lgo_present = !HELLO_LGO_BYTES.is_empty() && lgo_text.is_some();

    let start = {
        let emu = emu.clone();
        let interval_handle = interval_handle.clone();
        let uart_output = uart_output.clone();
        let status_msg = status_msg.clone();
        let halted = halted.clone();
        let running = running.clone();
        let instr_count = instr_count.clone();
        move || {
            *interval_handle.borrow_mut() = None;

            if !lgo_present {
                status_msg.set(
                    "Awaiting hello.lgo from dcftn (sw-cor24-fortran). \
                     The page is wired; running the demo is gated on the upstream artifact."
                        .into(),
                );
                halted.set(false);
                running.set(false);
                return;
            }
            let Some(content) = lgo_text else {
                status_msg.set("hello.lgo is not valid UTF-8 — refusing to load.".into());
                halted.set(false);
                running.set(false);
                return;
            };

            {
                let mut e = emu.borrow_mut();
                *e = EmulatorCore::new();
                if let Err(err) = e.load_lgo(content, None) {
                    status_msg.set(format!("Failed to load hello.lgo: {err}"));
                    halted.set(false);
                    running.set(false);
                    return;
                }
                e.resume();
            }

            uart_output.set(String::new());
            instr_count.set(0);
            halted.set(false);
            running.set(true);
            status_msg.set("Running".into());

            let emu = emu.clone();
            let uart_output = uart_output.clone();
            let status_msg = status_msg.clone();
            let halted = halted.clone();
            let running = running.clone();
            let instr_count = instr_count.clone();
            let interval_handle2 = interval_handle.clone();

            let interval = gloo_timers::callback::Interval::new(TICK_INTERVAL_MS, move || {
                let mut e = emu.borrow_mut();
                let batch = e.run_batch(MAX_INSTRUCTIONS_PER_TICK);
                uart_output.set(e.get_uart_output().to_string());
                instr_count.set(e.instructions_count());

                let stop = match batch.reason {
                    StopReason::Halted => {
                        halted.set(true);
                        status_msg.set("Halted".into());
                        true
                    }
                    StopReason::InvalidInstruction(op) => {
                        halted.set(true);
                        status_msg.set(format!(
                            "Invalid instruction: {op:#04x} at PC={:#06x}",
                            e.pc()
                        ));
                        true
                    }
                    StopReason::Paused => {
                        status_msg.set("Paused".into());
                        true
                    }
                    _ => false,
                };

                if stop {
                    running.set(false);
                    *interval_handle2.borrow_mut() = None;
                }
            });
            *interval_handle.borrow_mut() = Some(interval);
        }
    };

    {
        let start = start.clone();
        use_effect_with((), move |_| {
            start();
            || ()
        });
    }

    let on_run = {
        let start = start.clone();
        Callback::from(move |_: MouseEvent| start())
    };

    html! {
        <main style="display:flex; flex-direction:column; min-height:100vh; padding:24px; gap:16px;">
            <h1 style="font-size:1.4rem; color:#89b4fa;">
                {"FORTRAN Hello World on COR24"}
                <span style="font-size:0.85rem; color:#bac2de; margin-left:10px;">
                    {"hello.lgo running live in the embedded COR24 emulator (WASM)"}
                </span>
            </h1>

            <div style="display:flex; gap:16px; flex-wrap:wrap;">
                <div style="flex:1; min-width:320px; display:flex; flex-direction:column; gap:6px;">
                    <label style="color:#cdd6f4; font-weight:600; font-size:0.95rem;">
                        {"hello.f"}
                    </label>
                    <pre style="margin:0; padding:14px; background:#181825; \
                                border:1px solid #313244; border-radius:6px; \
                                color:#cdd6f4; font-family:'SF Mono','Fira Code',monospace; \
                                font-size:13px; line-height:1.5; white-space:pre; \
                                overflow:auto;">
                        { HELLO_F_SOURCE }
                    </pre>
                </div>

                <div style="flex:1; min-width:320px; display:flex; flex-direction:column; gap:6px;">
                    <label style="color:#cdd6f4; font-weight:600; font-size:0.95rem;">
                        {"UART output"}
                    </label>
                    <pre style="margin:0; padding:14px; background:#11111b; \
                                border:1px solid #313244; border-radius:6px; \
                                color:#a6e3a1; font-family:'SF Mono','Fira Code',monospace; \
                                font-size:13px; line-height:1.5; white-space:pre-wrap; \
                                min-height:140px; overflow:auto;">
                        { if uart_output.is_empty() && !*halted && !*running {
                            html! { <span style="color:#a6adc8;">{"(no output yet)"}</span> }
                        } else {
                            html! { {(*uart_output).clone()} }
                        }}
                    </pre>
                </div>
            </div>

            <div style="display:flex; gap:12px; align-items:center;">
                <button onclick={on_run}
                    style="padding:8px 18px; background:#89b4fa; color:#1e1e2e; \
                           border:none; border-radius:6px; font-size:1rem; font-weight:600; cursor:pointer;">
                    { if *running { "Restart" } else { "Run" } }
                </button>
                <span style="color:#bac2de; font-size:0.85rem;">{ &*status_msg }</span>
                <span style="color:#a6adc8; font-size:0.85rem; margin-left:auto;">
                    { format!("{} instructions", *instr_count) }
                </span>
            </div>

            if !lgo_present {
                <div style="padding:10px 14px; background:#181825; border:1px solid #585b70; \
                            border-radius:6px; color:#f9e2af; font-size:0.85rem;">
                    {"hello.lgo is a 0-byte placeholder. dcftn's "}
                    <a href="https://github.com/sw-embed/sw-cor24-fortran" target="_blank"
                        style="color:#89b4fa;">{"sw-cor24-fortran"}</a>
                    {" needs to ship the real artifact (Path A: hand-written hello.s assembled to hello.lgo). \
                      Once relayed and dropped in examples/hello.lgo here, the page runs the program for real."}
                </div>
            }

            <div style="display:flex; gap:8px; align-items:center; flex-wrap:wrap; \
                        font-size:0.85rem; color:#bac2de; padding-top:8px; border-top:1px solid #313244;">
                <span>{"\u{00a9} 2026 Michael A. Wright"}</span>
                <span>{"\u{00b7}"}</span>
                <span>{"MIT License"}</span>
                <span>{"\u{00b7}"}</span>
                <a href="https://github.com/sw-embed/web-sw-cor24-fortran" target="_blank"
                    style="color:#89b4fa; text-decoration:none;">{"Source"}</a>
                <span>{"\u{00b7}"}</span>
                <a href="https://github.com/sw-embed/sw-cor24-fortran" target="_blank"
                    style="color:#89b4fa; text-decoration:none;">{"Upstream compiler"}</a>
                <span>{"\u{00b7}"}</span>
                <a href="https://makerlisp.com" target="_blank"
                    style="color:#89b4fa; text-decoration:none;">{"COR24-TB"}</a>
                <span>{"\u{00b7}"}</span>
                <span>{ format!("{} \u{00b7} {} \u{00b7} {}",
                    env!("BUILD_HOST"),
                    env!("BUILD_SHA"),
                    env!("BUILD_TIMESTAMP"),
                ) }</span>
            </div>
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
