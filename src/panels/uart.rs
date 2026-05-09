use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UartPanelProps {
    pub output: AttrValue,
    pub running: bool,
    pub halted: bool,
    pub placeholder: AttrValue,
}

#[function_component(UartPanel)]
pub fn uart_panel(props: &UartPanelProps) -> Html {
    html! {
        <div style="flex:1; min-height:120px; display:flex; flex-direction:column; gap:4px;">
            <div style="color:#bac2de; font-size:0.8rem;">
                {"UART output"}
                if props.running { <span style="color:#a6adc8;">{" (running)"}</span> }
                if props.halted { <span style="color:#a6e3a1;">{" \u{2714} halted"}</span> }
            </div>
            <pre style="flex:1; min-height:80px; background:#11111b; color:#a6e3a1; padding:10px; \
                       border:1px solid #313244; border-radius:6px; \
                       font-family:monospace; font-size:13px; white-space:pre-wrap; \
                       overflow:auto; margin:0;">
                { if props.output.is_empty() && !props.running && !props.halted {
                    html! { <span style="color:#a6adc8;">{props.placeholder.clone()}</span> }
                } else {
                    html! { {props.output.clone()} }
                }}
            </pre>
        </div>
    }
}
