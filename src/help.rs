//! Modal dialog with Usage / Reference tabs. Triggered by the `[?]`
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
                       border-radius:8px; max-width:780px; width:90vw; \
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
                <li><b>{"Compile"}</b>{" \u{2014} runs dcftn's three-phase SNOBOL4 compiler chain. "}
                    {"For each phase a nested "}<code>{"cor24-emulator"}</code>{" loads "}
                    <code>{"snobol4.lgo"}</code>{" (dcsno's interpreter), drops the phase's "}
                    <code>{".sno"}</code>{" source at "}<code>{"0x080000"}</code>
                    {" and the phase's input at "}<code>{"0x090000"}</code>{", and captures the UART output:"}</li>
            </ol>
            <pre style="margin:0 0 12px 16px; padding:8px; background:#11111b; \
                        border:1px solid #313244; border-radius:4px; \
                        color:#cdd6f4; font-family:monospace; font-size:11.5px; \
                        line-height:1.45; white-space:pre; overflow:auto;">
{ "  your .f\n    \u{2192} normalize.sno  \u{2192} normalized statement records\n    \u{2192} classify.sno   \u{2192} records tagged with kind=PROGRAM/PRINT/STOP/...\n    \u{2192} emit_asm.sno   \u{2192} COR24 assembly (.s)" }
            </pre>
            <ol start="2" style="margin:0 0 16px 24px; padding:0;">
                <li><b>{"Assemble"}</b>{" \u{2014} runs "}<code>{"cor24-assembler"}</code>
                    {" over the "}<code>{".s"}</code>{" to produce machine code + listing."}</li>
                <li><b>{"Run"}</b>{" \u{2014} loads the bytes into a fresh "}<code>{"cor24-emulator"}</code>
                    {" and surfaces UART output."}</li>
            </ol>

            <h3 style="color:#cba6f7; margin:16px 0 8px 0;">{"Try it"}</h3>
            <ul style="margin:0 0 16px 24px; padding:0;">
                <li>{"Pick a demo from the dropdown to load source."}</li>
                <li>{"Edit if you like \u{2014} "}<code>{"PRINT *, 'whatever'"}</code>{" of any string compiles."}</li>
                <li>{"Click "}<b>{"Compile"}</b>{", "}<b>{"Assemble"}</b>{", "}<b>{"Run"}</b>{" in order, or just "}
                    <b>{"Run"}</b>{" to do all three."}</li>
                <li>{"Open the "}<i>{"Compiler trace"}</i>{" pane below the buttons to see what each phase emitted."}</li>
                <li>{"Refresh the browser to reset state."}</li>
            </ul>

            <h3 style="color:#cba6f7; margin:16px 0 8px 0;">{"What works today"}</h3>
            <p style="margin:0 0 12px 0;">
                {"dcftn shipped the m3-emit-hello milestone: "}<code>{"emit_asm.sno"}</code>{" handles "}
                <code>{"PROGRAM"}</code>{", "}<code>{"STOP"}</code>{", "}<code>{"END"}</code>{", and "}
                <code>{"PRINT *, 'string'"}</code>{". They're now working on m4 (integer PRINT). "}
                {"As later milestones land, refresh "}<code>{"assets/emit_asm.sno"}</code>
                {" from upstream and rebuild \u{2014} no code changes here."}
            </p>
            <p style="margin:0;">
                {"Today: "}<code>{"hello.f"}</code>{" compiles end-to-end. The other bundled demos "}
                {"run through normalize/classify successfully but emit_asm doesn't yet know how to "}
                {"emit code for "}<code>{"INTEGER"}</code>{", "}<code>{"DIMENSION"}</code>{", "}
                <code>{"DO"}</code>{", "}<code>{"GOTO"}</code>{", "}<code>{"IF"}</code>{", or integer "}
                <code>{"PRINT"}</code>{"."}
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
                There is "}<b>{"no"}</b>{" Rust-side Fortran parser. The compiler is dcftn's three "}
                {"SNOBOL4 programs, run on dcsno's SNOBOL4 interpreter, run on nested "}
                <code>{"cor24-emulator"}</code>{" instances. The wiring exactly mirrors what "}
                <code>{"scripts/fortran"}</code>{" does in "}
                <code>{"sw-cor24-fortran"}</code>{"."}
            </p>

            <h3 style="color:#cba6f7; margin:16px 0 8px 0;">{"Compiler phases"}</h3>
            <table style="border-collapse:collapse; width:100%; margin-bottom:16px; font-size:0.88rem;">
                <thead>
                    <tr style="background:#313244;">
                        <th style="text-align:left; padding:6px 10px; border:1px solid #45475a;">{"Phase"}</th>
                        <th style="text-align:left; padding:6px 10px; border:1px solid #45475a;">{"Input"}</th>
                        <th style="text-align:left; padding:6px 10px; border:1px solid #45475a;">{"Output"}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr><td style="padding:6px 10px; border:1px solid #45475a;"><code>{"normalize.sno"}</code></td>
                        <td style="padding:6px 10px; border:1px solid #45475a;">{"fixed-form .f source"}</td>
                        <td style="padding:6px 10px; border:1px solid #45475a;">{"normalized statement records (one per line: stmt<N> line=<M> label=<L> text=<text>)"}</td></tr>
                    <tr><td style="padding:6px 10px; border:1px solid #45475a;"><code>{"classify.sno"}</code></td>
                        <td style="padding:6px 10px; border:1px solid #45475a;">{"normalize output"}</td>
                        <td style="padding:6px 10px; border:1px solid #45475a;">{"same records with kind=<PROGRAM|PRINT|STOP|END|...> added"}</td></tr>
                    <tr><td style="padding:6px 10px; border:1px solid #45475a;"><code>{"emit_asm.sno"}</code></td>
                        <td style="padding:6px 10px; border:1px solid #45475a;">{"classify output"}</td>
                        <td style="padding:6px 10px; border:1px solid #45475a;">{"COR24 .s assembly"}</td></tr>
                </tbody>
            </table>

            <h3 style="color:#cba6f7; margin:16px 0 8px 0;">{"Bundled assets"}</h3>
            <table style="border-collapse:collapse; width:100%; margin-bottom:16px; font-size:0.88rem;">
                <thead>
                    <tr style="background:#313244;">
                        <th style="text-align:left; padding:6px 10px; border:1px solid #45475a;">{"File"}</th>
                        <th style="text-align:left; padding:6px 10px; border:1px solid #45475a;">{"Source"}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr><td style="padding:6px 10px; border:1px solid #45475a;"><code>{"assets/snobol4.lgo"}</code></td>
                        <td style="padding:6px 10px; border:1px solid #45475a;"><code>{"work/lib/cor24/snobol4.lgo"}</code>{" (sw-cor24-snobol4)"}</td></tr>
                    <tr><td style="padding:6px 10px; border:1px solid #45475a;"><code>{"assets/normalize.sno"}</code></td>
                        <td style="padding:6px 10px; border:1px solid #45475a;"><code>{"sw-cor24-fortran/snobol4/src/normalize.sno"}</code></td></tr>
                    <tr><td style="padding:6px 10px; border:1px solid #45475a;"><code>{"assets/classify.sno"}</code></td>
                        <td style="padding:6px 10px; border:1px solid #45475a;"><code>{"sw-cor24-fortran/snobol4/src/classify.sno"}</code></td></tr>
                    <tr><td style="padding:6px 10px; border:1px solid #45475a;"><code>{"assets/emit_asm.sno"}</code></td>
                        <td style="padding:6px 10px; border:1px solid #45475a;"><code>{"sw-cor24-fortran/snobol4/src/emit_asm.sno"}</code></td></tr>
                    <tr><td style="padding:6px 10px; border:1px solid #45475a;"><code>{"examples/*.f"}</code></td>
                        <td style="padding:6px 10px; border:1px solid #45475a;"><code>{"sw-cor24-fortran/examples"}</code></td></tr>
                </tbody>
            </table>

            <h3 style="color:#cba6f7; margin:16px 0 8px 0;">{"Refreshing the upstream"}</h3>
            <p style="margin:0;">
                {"As dcftn ships compiler milestones, refresh "}<code>{"assets/{normalize,classify,emit_asm}.sno"}</code>
                {" from "}<code>{"sw-cor24-fortran/snobol4/src/"}</code>{", refresh "}
                <code>{"assets/snobol4.lgo"}</code>{" from "}
                <code>{"work/lib/cor24/snobol4.lgo"}</code>{" when dcsno reships, then rebuild "}
                <code>{"./scripts/build-pages.sh"}</code>{" and re-deploy. No source changes required."}
            </p>
        </>
    }
}
