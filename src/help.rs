//! Modal dialog with Usage / Reference tabs documenting the
//! pipeline and current capability state. Triggered by the `[?]`
//! button in the header.

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HelpModalProps {
    pub open: bool,
    pub on_close: Callback<MouseEvent>,
}

#[derive(Clone, Copy, PartialEq)]
enum Tab {
    Usage,
    Reference,
}

#[function_component(HelpModal)]
pub fn help_modal(props: &HelpModalProps) -> Html {
    let tab = use_state(|| Tab::Usage);

    if !props.open {
        return html! {};
    }

    let backdrop_click = props.on_close.clone();
    let stop_click = Callback::from(|e: MouseEvent| e.stop_propagation());

    let select_usage = {
        let tab = tab.clone();
        Callback::from(move |_: MouseEvent| tab.set(Tab::Usage))
    };
    let select_reference = {
        let tab = tab.clone();
        Callback::from(move |_: MouseEvent| tab.set(Tab::Reference))
    };

    let tab_btn_style = |active: bool| -> String {
        let (bg, fg, bw) = if active {
            ("#313244", "#cdd6f4", "2px solid #89b4fa")
        } else {
            ("transparent", "#bac2de", "2px solid transparent")
        };
        format!(
            "padding:8px 16px; background:{bg}; color:{fg}; \
             border:none; border-bottom:{bw}; cursor:pointer; \
             font-size:0.9rem; font-weight:600;"
        )
    };

    let close_x = props.on_close.clone();

    html! {
        <div onclick={backdrop_click}
            style="position:fixed; inset:0; background:rgba(0,0,0,0.55); \
                   display:flex; align-items:center; justify-content:center; \
                   z-index:1000;">
            <div onclick={stop_click}
                style="background:#1e1e2e; color:#cdd6f4; border:1px solid #313244; \
                       border-radius:8px; max-width:760px; width:90vw; \
                       max-height:85vh; display:flex; flex-direction:column;">
                <div style="display:flex; align-items:center; justify-content:space-between; \
                            padding:12px 16px; border-bottom:1px solid #313244;">
                    <h2 style="margin:0; color:#89b4fa; font-size:1.05rem;">{"Help"}</h2>
                    <button onclick={close_x}
                        style="background:none; border:none; color:#bac2de; \
                               font-size:1.2rem; cursor:pointer; padding:0 4px;">
                        {"\u{2715}"}
                    </button>
                </div>
                <div style="display:flex; gap:0; padding:0 12px; border-bottom:1px solid #313244;">
                    <button onclick={select_usage}
                        style={tab_btn_style(*tab == Tab::Usage)}>
                        {"Usage"}
                    </button>
                    <button onclick={select_reference}
                        style={tab_btn_style(*tab == Tab::Reference)}>
                        {"Reference"}
                    </button>
                </div>
                <div style="padding:16px 20px; overflow:auto; line-height:1.55; font-size:0.92rem;">
                    if *tab == Tab::Usage {
                        { usage_content() }
                    } else {
                        { reference_content() }
                    }
                </div>
            </div>
        </div>
    }
}

fn usage_content() -> Html {
    html! {
        <>
            <h3 style="color:#cba6f7; margin:0 0 8px 0;">{"Pipeline"}</h3>
            <p style="margin:0 0 12px 0;">
                {"Three stages, all running in your browser:"}
            </p>
            <ol style="margin:0 0 16px 24px; padding:0;">
                <li><b>{"Compile"}</b>{" \u{2014} loads "}<code>{"snobol4.lgo"}</code>
                    {" (the SNOBOL4 interpreter, from sw-cor24-snobol4) into a nested COR24 emulator, feeds it "}
                    <code>{"fortran.sno"}</code>{" (the FTI-0 compiler from sw-cor24-fortran) followed by your "}
                    <code>{".f"}</code>{" source via UART, and captures the emitted COR24 assembly."}</li>
                <li><b>{"Assemble"}</b>{" \u{2014} runs "}<code>{"cor24-assembler"}</code>
                    {" over the "}<code>{".s"}</code>{" to produce machine code + listing."}</li>
                <li><b>{"Run"}</b>{" \u{2014} loads the bytes into a fresh "}<code>{"cor24-emulator"}</code>
                    {" and surfaces UART output."}</li>
            </ol>

            <h3 style="color:#cba6f7; margin:16px 0 8px 0;">{"Try it"}</h3>
            <ul style="margin:0 0 16px 24px; padding:0;">
                <li>{"Pick a demo from the dropdown to load the source."}</li>
                <li>{"Edit the source if you like."}</li>
                <li>{"Click "}<b>{"Compile"}</b>{" to see what fortran.sno emits, "}<b>{"Assemble"}</b>
                    {" to see the listing, "}<b>{"Run"}</b>{" to execute."}</li>
                <li>{"Refresh the browser to start over."}</li>
            </ul>

            <h3 style="color:#cba6f7; margin:16px 0 8px 0;">{"What works today"}</h3>
            <p style="margin:0 0 12px 0;">
                {"dcftn's "}<code>{"fortran.sno"}</code>{" is in research phase \u{2014} the current driver "}
                {"prints "}<code>{"\"FTI-0 compiler not yet implemented\""}</code>{" for any input. "}
                {"Only the canonical "}<code>{"hello.f"}</code>{" runs end-to-end today, via a "}
                {"Path-A short-circuit that swaps in dcftn's hand-written "}<code>{"hello.s"}</code>
                {" (mirroring what "}<code>{"scripts/fortran"}</code>{" does upstream)."}
            </p>
            <p style="margin:0 0 12px 0;">
                {"For the other demos (array1.f, goto1.f, sum10.f), the SNOBOL4 driver runs and "}
                {"prints its stub message; you can see it in the collapsible "}
                <i>{"\"SNOBOL4 driver output\""}</i>{" panel below the buttons. As dcftn ships "}
                <code>{"fortran.sno"}</code>{" phases (normalize, classify, expr, lower, emit), the demo will "}
                {"compile more of these inputs automatically \u{2014} just by refreshing "}
                <code>{"assets/fortran.sno"}</code>{" from upstream."}
            </p>
        </>
    }
}

fn reference_content() -> Html {
    html! {
        <>
            <h3 style="color:#cba6f7; margin:0 0 8px 0;">{"Architecture"}</h3>
            <p style="margin:0 0 12px 0;">
                {"This page is intentionally a "}<i>{"thin shell"}</i>{" over the upstream toolchain. \
                There is "}<b>{"no"}</b>{" Rust-side Fortran parser. The compiler is the SNOBOL4 program "}
                <code>{"fortran.sno"}</code>{", running on the SNOBOL4 interpreter "}
                <code>{"snobol4.lgo"}</code>{", running on a nested "}<code>{"cor24-emulator"}</code>{"."}
            </p>

            <pre style="margin:0 0 16px 0; padding:10px; background:#11111b; \
                        border:1px solid #313244; border-radius:4px; \
                        color:#cdd6f4; font-family:monospace; font-size:11.5px; \
                        line-height:1.45; white-space:pre; overflow:auto;">
{ "user .f source\n   |\n   v   cor24-emulator (this page) loads snobol4.lgo, sends fortran.sno + .f via UART\n   |\n   v   fortran.sno emits COR24 .s (today: stub message)\n   |\n   v   cor24-assembler (this page) -> bytes + listing\n   |\n   v   cor24-emulator (this page) runs bytes -> UART output" }
            </pre>

            <h3 style="color:#cba6f7; margin:16px 0 8px 0;">{"Bundled assets"}</h3>
            <table style="border-collapse:collapse; width:100%; margin-bottom:16px; font-size:0.88rem;">
                <thead>
                    <tr style="background:#313244;">
                        <th style="text-align:left; padding:6px 10px; border:1px solid #45475a;">{"File"}</th>
                        <th style="text-align:left; padding:6px 10px; border:1px solid #45475a;">{"Source"}</th>
                        <th style="text-align:left; padding:6px 10px; border:1px solid #45475a;">{"Purpose"}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr><td style="padding:6px 10px; border:1px solid #45475a;"><code>{"assets/snobol4.lgo"}</code></td>
                        <td style="padding:6px 10px; border:1px solid #45475a;"><code>{"sw-cor24-snobol4"}</code></td>
                        <td style="padding:6px 10px; border:1px solid #45475a;">{"SNOBOL4 interpreter (compiled to COR24)"}</td></tr>
                    <tr><td style="padding:6px 10px; border:1px solid #45475a;"><code>{"assets/fortran.sno"}</code></td>
                        <td style="padding:6px 10px; border:1px solid #45475a;"><code>{"sw-cor24-fortran"}</code></td>
                        <td style="padding:6px 10px; border:1px solid #45475a;">{"FTI-0 compiler in SNOBOL4 (today: research-phase stub)"}</td></tr>
                    <tr><td style="padding:6px 10px; border:1px solid #45475a;"><code>{"assets/hello.s"}</code></td>
                        <td style="padding:6px 10px; border:1px solid #45475a;"><code>{"sw-cor24-fortran"}</code></td>
                        <td style="padding:6px 10px; border:1px solid #45475a;">{"Path-A fixture: dcftn's hand-written hello.s"}</td></tr>
                    <tr><td style="padding:6px 10px; border:1px solid #45475a;"><code>{"examples/*.f"}</code></td>
                        <td style="padding:6px 10px; border:1px solid #45475a;"><code>{"sw-cor24-fortran"}</code></td>
                        <td style="padding:6px 10px; border:1px solid #45475a;">{"FTI-0 demo programs (hello, array1, goto1, sum10)"}</td></tr>
                </tbody>
            </table>

            <h3 style="color:#cba6f7; margin:16px 0 8px 0;">{"Path A short-circuit"}</h3>
            <p style="margin:0 0 12px 0;">
                {"When the source matches dcftn's canonical "}<code>{"hello.f"}</code>{" exactly, the demo "}
                {"swaps in their pre-baked "}<code>{"hello.s"}</code>{" instead of using the SNOBOL4 driver's "}
                {"output. This mirrors the upstream "}<code>{"scripts/fortran"}</code>
                {" which short-circuits hello.f because the full FTI-0 compiler isn't ready yet. The .s pane "}
                {"shows a "}<i>{"\"\u{00b7} Path-A short-circuit\""}</i>{" badge when this happens."}
            </p>

            <h3 style="color:#cba6f7; margin:16px 0 8px 0;">{"Refreshing fortran.sno"}</h3>
            <p style="margin:0;">
                {"As dcftn ships compiler phases, refresh "}<code>{"assets/fortran.sno"}</code>
                {" from "}<code>{"sw-cor24-fortran/snobol4/src/driver.sno"}</code>{" (or whatever bundled "}
                {"form they ship), rebuild "}<code>{"./scripts/build-pages.sh"}</code>{", and re-deploy. "}
                {"No code changes here are required."}
            </p>
        </>
    }
}
