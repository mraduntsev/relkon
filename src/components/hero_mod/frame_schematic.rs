use leptos::prelude::*;

#[component]
pub fn FrameSchematic() -> impl IntoView {
    view! {
        <svg viewBox="0 0 640 480" class="w-full h-auto text-[#2B4E9B] dark:text-sky-300/90" fill="none" stroke="currentColor" stroke-width="1.6" role="img" aria-label="Схема стальной рамы каркаса">
            <defs>
                <marker id="arr" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
                    <path d="M0,0 L10,5 L0,10 z" fill="currentColor" stroke="none"/>
                </marker>
            </defs>

            <g stroke-dasharray="6 6" opacity=".55">
                <line class="sf" style="animation-delay:.1s" x1="90" y1="388" x2="90" y2="424"/>
                <line class="sf" style="animation-delay:.1s" x1="550" y1="388" x2="550" y2="424"/>
            </g>
            <g class="sf" style="animation-delay:.2s" stroke="none" fill="currentColor" font-family="JetBrains Mono, monospace" font-size="12">
                <circle cx="90" cy="440" r="14" fill="none" stroke="currentColor" stroke-width="1.4"/>
                <circle cx="550" cy="440" r="14" fill="none" stroke="currentColor" stroke-width="1.4"/>
                <text x="90" y="444" text-anchor="middle">"1"</text>
                <text x="550" y="444" text-anchor="middle">"2"</text>
            </g>

            <line class="dw" pathLength="1" style="animation-delay:.25s" x1="90" y1="380" x2="90" y2="210"/>
            <line class="dw" pathLength="1" style="animation-delay:.35s" x1="550" y1="380" x2="550" y2="210"/>
            <line class="dw" pathLength="1" style="animation-delay:.5s" x1="70" y1="380" x2="110" y2="380"/>
            <line class="dw" pathLength="1" style="animation-delay:.5s" x1="530" y1="380" x2="570" y2="380"/>

            <line class="dw" pathLength="1" style="animation-delay:.6s" x1="90" y1="210" x2="550" y2="210"/>
            <line class="dw" pathLength="1" style="animation-delay:.75s" x1="90" y1="210" x2="320" y2="130"/>
            <line class="dw" pathLength="1" style="animation-delay:.85s" x1="320" y1="130" x2="550" y2="210"/>
            <line class="dw" pathLength="1" style="animation-delay:1s" x1="205" y1="210" x2="205" y2="170"/>
            <line class="dw" pathLength="1" style="animation-delay:1s" x1="320" y1="210" x2="320" y2="130"/>
            <line class="dw" pathLength="1" style="animation-delay:1s" x1="435" y1="210" x2="435" y2="170"/>
            <line class="dw" pathLength="1" style="animation-delay:1.1s" x1="205" y1="210" x2="320" y2="130"/>
            <line class="dw" pathLength="1" style="animation-delay:1.1s" x1="435" y1="210" x2="320" y2="130"/>

            <g class="sf" style="animation-delay:1.25s">
                <circle cx="90" cy="210" r="4"/><circle cx="205" cy="170" r="4"/><circle cx="320" cy="130" r="4"/>
                <circle cx="435" cy="170" r="4"/><circle cx="550" cy="210" r="4"/><circle cx="205" cy="210" r="4"/><circle cx="435" cy="210" r="4"/>
            </g>

            <g class="dw" pathLength="1" style="animation-delay:1.3s" stroke-width="1">
                <line x1="90" y1="200" x2="90" y2="92"/><line x1="550" y1="200" x2="550" y2="92"/>
                <line x1="90" y1="100" x2="550" y2="100" marker-start="url(#arr)" marker-end="url(#arr)"/>
            </g>
            <text class="sf" style="animation-delay:1.45s" x="320" y="88" text-anchor="middle" stroke="none" fill="currentColor" font-family="JetBrains Mono, monospace" font-size="13">"24 000"</text>

            <g class="dw" pathLength="1" style="animation-delay:1.4s" stroke-width="1">
                <line x1="556" y1="210" x2="600" y2="210"/><line x1="576" y1="380" x2="600" y2="380"/>
                <line x1="594" y1="210" x2="594" y2="380" marker-start="url(#arr)" marker-end="url(#arr)"/>
            </g>
            <text class="sf" style="animation-delay:1.55s" x="612" y="300" stroke="none" fill="currentColor" font-family="JetBrains Mono, monospace" font-size="12" transform="rotate(-90 612 300)" text-anchor="middle">"8 400"</text>

            <polyline class="dw" pathLength="1" style="animation-delay:1.5s" points="435,170 470,140 552,140" stroke-width="1"/>
            <text class="sf" style="animation-delay:1.65s" x="470" y="132" stroke="none" fill="currentColor" font-family="JetBrains Mono, monospace" font-size="11">"уз.3 · см. · л.11"</text>

            <g class="sf" style="animation-delay:1.7s">
                <rect x="20" y="20" width="170" height="70" stroke-width="1.2"/>
                <line x1="20" y1="44" x2="190" y2="44" stroke-width="1"/>
                <line x1="20" y1="67" x2="190" y2="67" stroke-width="1"/>
                <g stroke="none" fill="currentColor" font-family="JetBrains Mono, monospace" font-size="11">
                    <text x="30" y="37">"СТАЛЬ: С255 / С345"</text>
                    <text x="30" y="60">"ГОСТ 27772-2021"</text>
                    <text x="30" y="83">"БОЛТЫ: М24, кл. 10.9"</text>
                </g>
            </g>
        </svg>
    }
}
