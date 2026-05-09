//! FORTRAN Hello World on COR24 — minimal Yew/WASM live demo.
//!
//! Embeds dcftn's pre-built `examples/hello.lgo` (produced by their
//! `pr/fortran-hello-world` saga, Path A) and runs it inside
//! `cor24_emulator::EmulatorCore`. The page shows the canonical
//! `examples/hello.f` source on the left and the program's UART output
//! on the right.
//!
//! No in-browser compilation, no editor — per the brief
//! `tools/briefs/dwftn-hello-world-demo.md`. The Fortran compiler is
//! the upstream sw-cor24-fortran's responsibility; this page is a thin
//! demo of its output.

use std::cell::RefCell;
use std::rc::Rc;

use cor24_emulator::{EmulatorCore, StopReason};
use yew::prelude::*;

const HELLO_F_SOURCE: &str = include_str!("../examples/hello.f");
const HELLO_LGO_BYTES: &[u8] = include_bytes!("../examples/hello.lgo");

const TICK_INTERVAL_MS: u32 = 16;
const RUN_BATCH: u64 = 100_000;

#[function_component(App)]
fn app() -> Html {
    let uart_output = use_state(String::new);
    let halted = use_state(|| false);
    let running = use_state(|| false);
    let status_msg = use_state(|| String::from("Loading hello.lgo..."));

    let emu: Rc<RefCell<EmulatorCore>> = use_mut_ref(EmulatorCore::new);
    let interval_handle = use_mut_ref(|| None::<gloo_timers::callback::Interval>);

    let start = {
        let emu = emu.clone();
        let interval_handle = interval_handle.clone();
        let uart_output = uart_output.clone();
        let halted = halted.clone();
        let running = running.clone();
        let status_msg = status_msg.clone();
        move || {
            *interval_handle.borrow_mut() = None;

            let lgo_text = match std::str::from_utf8(HELLO_LGO_BYTES) {
                Ok(s) => s,
                Err(e) => {
                    status_msg.set(format!("hello.lgo is not valid UTF-8: {e}"));
                    return;
                }
            };

            {
                let mut e = emu.borrow_mut();
                *e = EmulatorCore::new();
                if let Err(err) = e.load_lgo(lgo_text, None) {
                    status_msg.set(format!("Failed to load hello.lgo: {err}"));
                    return;
                }
                e.resume();
            }

            uart_output.set(String::new());
            halted.set(false);
            running.set(true);
            status_msg.set("Running".into());

            let emu = emu.clone();
            let uart_output = uart_output.clone();
            let halted = halted.clone();
            let running = running.clone();
            let status_msg = status_msg.clone();
            let interval_handle2 = interval_handle.clone();

            let interval = gloo_timers::callback::Interval::new(TICK_INTERVAL_MS, move || {
                let mut e = emu.borrow_mut();
                let batch = e.run_batch(RUN_BATCH);
                uart_output.set(e.get_uart_output().to_string());

                let stop = match batch.reason {
                    StopReason::Halted => {
                        halted.set(true);
                        status_msg.set("Halted".into());
                        true
                    }
                    StopReason::InvalidInstruction(op) => {
                        halted.set(true);
                        status_msg.set(format!(
                            "Invalid instruction {op:#04x} at PC={:#06x}",
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
        <main style="display:flex; flex-direction:column; min-height:100vh; padding:24px; gap:16px; \
                     box-sizing:border-box;">
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
                            html! { (*uart_output).clone() }
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
            </div>

            <div style="display:flex; gap:8px; align-items:center; flex-wrap:wrap; \
                        font-size:0.85rem; color:#bac2de; padding-top:8px; border-top:1px solid #313244;">
                <span>{"\u{00a9} 2026 Michael A. Wright"}</span>
                <span>{"\u{00b7}"}</span>
                <span>{"MIT"}</span>
                <span>{"\u{00b7}"}</span>
                <a href="https://github.com/sw-embed/web-sw-cor24-fortran" target="_blank"
                    style="color:#89b4fa; text-decoration:none;">{"Source"}</a>
                <span>{"\u{00b7}"}</span>
                <a href="https://github.com/sw-embed/sw-cor24-fortran" target="_blank"
                    style="color:#89b4fa; text-decoration:none;">{"Upstream Fortran compiler"}</a>
                <span>{"\u{00b7}"}</span>
                <a href="https://makerlisp.com" target="_blank"
                    style="color:#89b4fa; text-decoration:none;">{"COR24-TB"}</a>
                <span>{"\u{00b7}"}</span>
                <span>{ format!("{} \u{00b7} {} \u{00b7} {}",
                    env!("BUILD_HOST"), env!("BUILD_SHA"), env!("BUILD_TIMESTAMP"),
                ) }</span>
            </div>
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
