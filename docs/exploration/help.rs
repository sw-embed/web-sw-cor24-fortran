//! Modal dialog with Usage / Reference tabs documenting the FTI-0
//! subset this demo supports. Triggered by the `[?]` button in the
//! header.

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
                       border-radius:8px; max-width:720px; width:90vw; \
                       max-height:80vh; display:flex; flex-direction:column;">
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
            <p style="margin:0 0 12px 0;">{"This page runs a complete Fortran toolchain in your browser:"}</p>
            <ol style="margin:0 0 16px 24px; padding:0;">
                <li><b>{"Compile"}</b>{" — turns your "}<code>{".f"}</code>{" source into COR24 assembly ("}<code>{".s"}</code>{")."}</li>
                <li><b>{"Assemble"}</b>{" — runs "}<code>{"cor24-assembler"}</code>{" over the "}<code>{".s"}</code>{" to produce machine code + listing."}</li>
                <li><b>{"Run"}</b>{" — loads the bytes into "}<code>{"cor24-emulator"}</code>{" and prints UART output."}</li>
            </ol>

            <h3 style="color:#cba6f7; margin:16px 0 8px 0;">{"Try it"}</h3>
            <ul style="margin:0 0 16px 24px; padding:0;">
                <li>{"Pick a demo from the dropdown to load fresh source."}</li>
                <li>{"Edit the strings in "}<code>{"PRINT *, '...'"}</code>{" — the program will compile, assemble, and run with your changes."}</li>
                <li>{"Add or remove "}<code>{"PRINT"}</code>{" statements; each gets its own UART line."}</li>
                <li>{"Refresh the browser to start over."}</li>
            </ul>

            <h3 style="color:#cba6f7; margin:16px 0 8px 0;">{"Buttons"}</h3>
            <ul style="margin:0 0 16px 24px; padding:0;">
                <li><b>{"Compile"}</b>{" — only runs stage 1 (.s in the middle pane)."}</li>
                <li><b>{"Assemble"}</b>{" — runs stages 1+2 (.s and listing)."}</li>
                <li><b>{"Run"}</b>{" — runs all three stages and shows UART output."}</li>
                <li><b>{"Stop"}</b>{" — pause a running program."}</li>
            </ul>
        </>
    }
}

fn reference_content() -> Html {
    html! {
        <>
            <h3 style="color:#cba6f7; margin:0 0 8px 0;">{"Supported FTI-0 subset"}</h3>
            <p style="margin:0 0 12px 0;">
                {"This demo's compiler is intentionally tiny. It accepts:"}
            </p>
            <table style="border-collapse:collapse; width:100%; margin-bottom:16px; font-size:0.9rem;">
                <thead>
                    <tr style="background:#313244;">
                        <th style="text-align:left; padding:6px 10px; border:1px solid #45475a;">{"Construct"}</th>
                        <th style="text-align:left; padding:6px 10px; border:1px solid #45475a;">{"Notes"}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr><td style="padding:6px 10px; border:1px solid #45475a;"><code>{"C ..."}</code></td>
                        <td style="padding:6px 10px; border:1px solid #45475a;">{"Column-1 comment ('C', 'c', or '*'). Whole line ignored."}</td></tr>
                    <tr><td style="padding:6px 10px; border:1px solid #45475a;"><code>{"PROGRAM name"}</code></td>
                        <td style="padding:6px 10px; border:1px solid #45475a;">{"Optional. Consumed; emits no code."}</td></tr>
                    <tr><td style="padding:6px 10px; border:1px solid #45475a;"><code>{"PRINT *, arg1, arg2, ..."}</code></td>
                        <td style="padding:6px 10px; border:1px solid #45475a;">
                            {"Writes args (space-separated) + newline. Each arg is either "}
                            <code>{"'string'"}</code>
                            {" or an integer expression: literals, "}
                            <code>{"+ - * /"}</code>
                            {", parens, unary minus. Expressions are evaluated at compile time."}
                        </td></tr>
                    <tr><td style="padding:6px 10px; border:1px solid #45475a;"><code>{"STOP"}</code> {", "} <code>{"END"}</code></td>
                        <td style="padding:6px 10px; border:1px solid #45475a;">{"Consumed. Generated assembly halts after the last PRINT."}</td></tr>
                    <tr><td style="padding:6px 10px; border:1px solid #45475a;">{"Statement labels"}</td>
                        <td style="padding:6px 10px; border:1px solid #45475a;">{"Columns 1–5 may hold a label; ignored."}</td></tr>
                </tbody>
            </table>

            <h3 style="color:#cba6f7; margin:16px 0 8px 0;">{"String escapes"}</h3>
            <p style="margin:0 0 12px 0;">
                {"Inside a single-quoted string, Fortran's classic "}
                <code>{"''"}</code>
                {" escape produces a single quote. e.g. "}
                <code>{"'It''s working!'"}</code>
                {" → "}
                <code>{"It's working!"}</code>
                {"."}
            </p>

            <h3 style="color:#cba6f7; margin:16px 0 8px 0;">{"Not supported (yet)"}</h3>
            <ul style="margin:0 0 16px 24px; padding:0;">
                <li><code>{"INTEGER"}</code>{", "}<code>{"REAL"}</code>{", and other type declarations"}</li>
                <li>{"Variables, expressions, arithmetic"}</li>
                <li><code>{"DO"}</code>{" loops, "}<code>{"GOTO"}</code>{", "}<code>{"IF"}</code></li>
                <li><code>{"DIMENSION"}</code>{" / arrays"}</li>
                <li>{"Continuation lines (column-6 marker)"}</li>
                <li>{"Numeric output (no integer/real formatting)"}</li>
            </ul>
            <p style="margin:0 0 12px 0; color:#bac2de;">
                {"The full FTI-0 compiler — written in SNOBOL4 — lives upstream at "}
                <a href="https://github.com/sw-embed/sw-cor24-fortran" target="_blank"
                    style="color:#89b4fa;">{"sw-cor24-fortran"}</a>
                {". As that compiler matures, this demo's stage-1 will be replaced by running the SNOBOL4 image in a nested COR24 emulator."}
            </p>

            <h3 style="color:#cba6f7; margin:16px 0 8px 0;">{"COR24 host"}</h3>
            <p style="margin:0;">
                {"The cor24 emulator running this demo is the same Rust crate ("}
                <code>{"cor24-emulator"}</code>
                {") used by the standalone "}
                <code>{"cor24-emu"}</code>
                {" CLI. UART output goes to memory-mapped I/O at "}
                <code>{"0xFF0100"}</code>
                {" (data) / "}
                <code>{"0xFF0101"}</code>
                {" (status). The compiler's emitted "}
                <code>{".s"}</code>
                {" uses the same conventions as "}
                <a href="https://github.com/sw-embed/sw-cor24-x-tinyc" target="_blank"
                    style="color:#89b4fa;">{"sw-cor24-x-tinyc"}</a>
                {"."}
            </p>
        </>
    }
}
