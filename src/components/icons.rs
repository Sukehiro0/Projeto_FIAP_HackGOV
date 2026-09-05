use dioxus::prelude::*;

/// Catálogo de ícones usados em toda a aplicação.
///
/// Por que SVG inline em vez de emoji ou de uma biblioteca de ícones via CDN:
/// - Emoji não tem identidade visual: cada sistema operacional desenha um
///   símbolo diferente (Android, iOS, Windows, Linux todos renderizam "🛡️"
///   de um jeito distinto), o que quebra a consistência visual esperada de
///   um portal de governo e não segue o Design System gov.br (DSGov).
/// - Carregar uma fonte de ícones ou um script de terceiros (ex: CDN de
///   ícones) criaria uma dependência externa desnecessária: mais uma origem
///   para configurar no CSP, mais uma superfície de ataque de cadeia de
///   suprimentos (supply chain) e mais uma requisição de rede.
/// - SVG inline, gerado localmente e sem `use dangerouslySetInnerHTML`
///   equivalente, é compilado dentro do bundle .wasm: nada é buscado de
///   fora, então não há o que interceptar ou substituir.
#[derive(Clone, Copy, PartialEq)]
pub enum IconKind {
    Shield,
    Bell,
    Menu,
    Close,
    Search,
    User,
    ArrowRight,
    ArrowLeft,
    Check,
    CheckCircle,
    Warning,
    Calendar,
    Star,
    StarOutline,
    Sparkle,
    Contrast,
    Accessibility,
    Speaker,
    BookOpen,
    Camera,
    Phone,
    Mail,
    Landmark,
    Building,
    Lock,
    Key,
    Bug,
    Hourglass,
    Chat,
    HelpCircle,
    LifeBuoy,
    Document,
    IdCard,
    Car,
    Plane,
    Signature,
    Briefcase,
    HealthCross,
    GraduationCap,
    Clock,
    Leaf,
    Users,
    Flask,
    Scale,
    Tool,
    Globe,
    Palette,
    Fingerprint,
    Activity,
    Money,
    Ballot,
    Lightbulb,
    Trash,
    Cone,
    XLogo,
    Facebook,
    Instagram,
    LinkedIn,
    YouTube,
}

/// Ícone SVG acessível. `class` controla tamanho/cor via Tailwind
/// (ex.: "w-5 h-5 text-govbr-blue"); a cor do traço segue `currentColor`,
/// então herda a cor de texto do elemento pai.
#[component]
pub fn Icon(
    kind: IconKind,
    #[props(default = "w-5 h-5".to_string())] class: String,
) -> Element {
    rsx! {
        svg {
            class: "{class} shrink-0",
            "viewBox": "0 0 24 24",
            "fill": "none",
            "stroke": "currentColor",
            "stroke-width": "1.8",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            "aria-hidden": "true",
            focusable: "false",
            {icon_body(kind)}
        }
    }
}

fn icon_body(kind: IconKind) -> Element {
    match kind {
        IconKind::Shield => rsx! {
            path { "d": "M12 3l7 3v5c0 5-3 9-7 10-4-1-7-5-7-10V6z" }
        },
        IconKind::Bell => rsx! {
            path { "d": "M6 8a6 6 0 1 1 12 0c0 4.5 1.5 6 2 6.5H4C4.5 14.5 6 12.5 6 8z" }
            path { "d": "M9.5 18.5a2.5 2.5 0 0 0 5 0" }
        },
        IconKind::Menu => rsx! {
            path { "d": "M4 6h16M4 12h16M4 18h16" }
        },
        IconKind::Close => rsx! {
            path { "d": "M5 5l14 14M19 5L5 19" }
        },
        IconKind::Search => rsx! {
            circle { "cx": "11", "cy": "11", "r": "7" }
            path { "d": "M20 20l-4.3-4.3" }
        },
        IconKind::User => rsx! {
            circle { "cx": "12", "cy": "8", "r": "4" }
            path { "d": "M4 20c0-4.4 3.6-7 8-7s8 2.6 8 7" }
        },
        IconKind::ArrowRight => rsx! {
            path { "d": "M4 12h14M13 6l7 6-7 6" }
        },
        IconKind::ArrowLeft => rsx! {
            path { "d": "M20 12H6M11 6l-7 6 7 6" }
        },
        IconKind::Check => rsx! {
            path { "d": "M4 12.5l5 5L20 6" }
        },
        IconKind::CheckCircle => rsx! {
            circle { "cx": "12", "cy": "12", "r": "9" }
            path { "d": "M8 12.5l2.6 2.6L16 9" }
        },
        IconKind::Warning => rsx! {
            path { "d": "M12 3L22 20H2z" }
            path { "d": "M12 9.5v4.5" }
            circle { "cx": "12", "cy": "17", "r": "0.9", "fill": "currentColor", "stroke": "none" }
        },
        IconKind::Calendar => rsx! {
            rect { "x": "3", "y": "5", "width": "18", "height": "16", "rx": "2" }
            path { "d": "M3 10h18M8 3v4M16 3v4" }
        },
        IconKind::Star => rsx! {
            path {
                "fill": "currentColor",
                "stroke": "none",
                "d": "M12 3l2.6 5.9 6.4.6-4.8 4.3 1.4 6.3L12 17l-5.6 3.1 1.4-6.3L3 9.5l6.4-.6z",
            }
        },
        IconKind::StarOutline => rsx! {
            path { "d": "M12 3l2.6 5.9 6.4.6-4.8 4.3 1.4 6.3L12 17l-5.6 3.1 1.4-6.3L3 9.5l6.4-.6z" }
        },
        IconKind::Sparkle => rsx! {
            path {
                "fill": "currentColor",
                "stroke": "none",
                "d": "M12 3l1.4 4.2L18 8.5l-4.6 1.3L12 14l-1.4-4.2L6 8.5l4.6-1.3z",
            }
        },
        IconKind::Contrast => rsx! {
            circle { "cx": "12", "cy": "12", "r": "9" }
            path { "fill": "currentColor", "stroke": "none", "d": "M12 3a9 9 0 0 1 0 18z" }
        },
        IconKind::Accessibility => rsx! {
            circle { "cx": "12", "cy": "12", "r": "9" }
            circle { "cx": "12", "cy": "7.7", "r": "1.4", "fill": "currentColor", "stroke": "none" }
            path { "d": "M7.5 10.5L12 12l4.5-1.5M12 12v5M9.5 20l2.5-3 2.5 3" }
        },
        IconKind::Speaker => rsx! {
            path { "d": "M4 9v6h4l5 4V5L8 9H4z" }
            path { "d": "M16.3 9.3a4 4 0 0 1 0 5.5M19 6.8a8 8 0 0 1 0 10.4" }
        },
        IconKind::BookOpen => rsx! {
            path { "d": "M4 5.5c2.2-1.2 5.2-1.2 8 .8v13c-2.8-2-5.8-2-8-.8zM20 5.5c-2.2-1.2-5.2-1.2-8 .8v13c2.8-2 5.8-2 8-.8z" }
        },
        IconKind::Camera => rsx! {
            rect { "x": "3", "y": "7", "width": "18", "height": "13", "rx": "2" }
            path { "d": "M8 7l1.4-2.5h5.2L16 7" }
            circle { "cx": "12", "cy": "13.5", "r": "3.3" }
        },
        IconKind::Phone => rsx! {
            path { "d": "M6 3h3l1.4 4.5L8.3 9.6c1 2.9 3.2 5.1 6.1 6.1l2.1-2.1L21 15v3c0 1.1-.9 2-2 2C11 20 4 13 4 5c0-1.1.9-2 2-2z" }
        },
        IconKind::Mail => rsx! {
            rect { "x": "3", "y": "5", "width": "18", "height": "14", "rx": "2" }
            path { "d": "M3 6.5l9 6.5 9-6.5" }
        },
        IconKind::Landmark => rsx! {
            path { "d": "M2 10l10-6 10 6z" }
            path { "d": "M4 10v10M9 10v10M15 10v10M20 10v10M2 21h20" }
        },
        IconKind::Building => rsx! {
            rect { "x": "5", "y": "3", "width": "14", "height": "18" }
            path { "d": "M9 7h1M14 7h1M9 11h1M14 11h1M9 15h1M14 15h1" }
            path { "d": "M10 21v-4h4v4" }
        },
        IconKind::Lock => rsx! {
            rect { "x": "5", "y": "11", "width": "14", "height": "10", "rx": "2" }
            path { "d": "M8 11V8a4 4 0 0 1 8 0v3" }
        },
        IconKind::Key => rsx! {
            circle { "cx": "8", "cy": "8", "r": "4" }
            path { "d": "M11 11L20 20M15.5 15.5l2-2M18.5 12.5l2 2" }
        },
        IconKind::Bug => rsx! {
            circle { "cx": "12", "cy": "13", "r": "5" }
            path { "d": "M9 8l-2-2M15 8l2-2M7 13H3M21 13h-4M9 18l-2 2M15 18l2 2M10 9h4" }
        },
        IconKind::Hourglass => rsx! {
            path { "d": "M6 3h12M6 21h12M7 3c0 4.5 3 6.5 5 9-2 2.5-5 4.5-5 9M17 3c0 4.5-3 6.5-5 9 2 2.5 5 4.5 5 9" }
        },
        IconKind::Chat => rsx! {
            rect { "x": "4", "y": "5", "width": "16", "height": "11", "rx": "2" }
            path { "d": "M8 16l-2 4v-4" }
        },
        IconKind::HelpCircle => rsx! {
            circle { "cx": "12", "cy": "12", "r": "9" }
            path { "d": "M9.5 9.3a2.6 2.6 0 1 1 3.7 2.4c-1 .5-1.7 1.1-1.7 2.3" }
            circle { "cx": "12", "cy": "17.2", "r": "0.9", "fill": "currentColor", "stroke": "none" }
        },
        IconKind::LifeBuoy => rsx! {
            circle { "cx": "12", "cy": "12", "r": "9" }
            circle { "cx": "12", "cy": "12", "r": "4" }
            path { "d": "M7 7l2.5 2.5M17 7l-2.5 2.5M7 17l2.5-2.5M17 17l-2.5-2.5" }
        },
        IconKind::Document => rsx! {
            path { "d": "M7 3h7l4 4v14H7z" }
            path { "d": "M14 3v4h4M9 12h6M9 16h6" }
        },
        IconKind::IdCard => rsx! {
            rect { "x": "3", "y": "5", "width": "18", "height": "14", "rx": "2" }
            circle { "cx": "8.5", "cy": "11", "r": "2" }
            path { "d": "M6 16c0-1.7 1.1-3 2.5-3s2.5 1.3 2.5 3M13.5 9h4M13.5 12.5h4M13.5 16h2.5" }
        },
        IconKind::Car => rsx! {
            path { "d": "M4 16l1.4-5.3A2 2 0 0 1 7.3 9.2h9.4a2 2 0 0 1 1.9 1.5L20 16" }
            rect { "x": "2.5", "y": "16", "width": "19", "height": "3.2", "rx": "1" }
            circle { "cx": "7", "cy": "19.6", "r": "1.3", "fill": "currentColor", "stroke": "none" }
            circle { "cx": "17", "cy": "19.6", "r": "1.3", "fill": "currentColor", "stroke": "none" }
        },
        IconKind::Plane => rsx! {
            path { "d": "M22 2L11 13M22 2l-7 20-4-9-9-4z" }
        },
        IconKind::Signature => rsx! {
            path { "d": "M3 18c4.2 0 5.2-9.5 9.2-9.5 3 0 .8 6.2-2.2 8.2s-6 1.8-6 1.8c3.2 0 6.5-1 8.6-3.7" }
            path { "d": "M16.3 8.7l3-3 2 2-3 3-2.4.4z" }
        },
        IconKind::Briefcase => rsx! {
            rect { "x": "3", "y": "8", "width": "18", "height": "12", "rx": "2" }
            path { "d": "M8 8V6a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2M3 13h18" }
        },
        IconKind::HealthCross => rsx! {
            rect { "x": "4", "y": "4", "width": "16", "height": "16", "rx": "3" }
            path { "d": "M12 8v8M8 12h8" }
        },
        IconKind::GraduationCap => rsx! {
            path { "d": "M2 9l10-5 10 5-10 5-10-5z" }
            path { "d": "M6 11.3v4.7c0 1.5 2.7 3 6 3s6-1.5 6-3v-4.7" }
            path { "d": "M22 9v6" }
        },
        IconKind::Clock => rsx! {
            circle { "cx": "12", "cy": "12", "r": "9" }
            path { "d": "M12 7v5l4 2" }
        },
        IconKind::Leaf => rsx! {
            path { "d": "M20 4C10 4 4 10 4 18c8 0 14-6 14-14z" }
            path { "d": "M6 18c4-4 8-8 12-12" }
        },
        IconKind::Users => rsx! {
            circle { "cx": "9", "cy": "8", "r": "3" }
            path { "d": "M3 20c0-3.3 2.7-6 6-6s6 2.7 6 6" }
            circle { "cx": "17.5", "cy": "9", "r": "2.3" }
            path { "d": "M15.3 20c0-2.6 1-4.6 3-5.6 1.9 1 3.2 3 3.2 5.6" }
        },
        IconKind::Flask => rsx! {
            path { "d": "M9 3h6M10 3v6l-5.3 9.3A1.8 1.8 0 0 0 6.3 21h11.4a1.8 1.8 0 0 0 1.6-2.7L14 9V3" }
            path { "d": "M8 15.5h8" }
        },
        IconKind::Scale => rsx! {
            path { "d": "M12 3v18M5 21h14M4 8h16" }
            path { "d": "M4 8l-2 6a3 3 0 0 0 6 0z" }
            path { "d": "M20 8l2 6a3 3 0 0 1-6 0z" }
        },
        IconKind::Tool => rsx! {
            path { "d": "M14.7 5.6a3.8 3.8 0 1 0 3.7 3.7L21 6.7l-2.4-2.4z" }
            path { "d": "M12.5 11.5L4.5 19.5a1.7 1.7 0 0 0 2.4 2.4l8-8" }
        },
        IconKind::Globe => rsx! {
            circle { "cx": "12", "cy": "12", "r": "9" }
            path { "d": "M3 12h18" }
            path { "d": "M12 3c2.5 2.5 4 5.6 4 9s-1.5 6.5-4 9c-2.5-2.5-4-5.6-4-9s1.5-6.5 4-9z" }
        },
        IconKind::Palette => rsx! {
            path { "d": "M12 3a9 9 0 1 0 0 18c1.4 0 1.9-.8 1.9-1.8 0-.5-.2-.9-.5-1.3-.3-.4-.5-.7-.5-1.1 0-.8.7-1.4 1.5-1.4h1.8a4.2 4.2 0 0 0 4.2-4.2C20.4 6.3 16.7 3 12 3z" }
            circle { "cx": "8", "cy": "10", "r": "1.1", "fill": "currentColor", "stroke": "none" }
            circle { "cx": "8", "cy": "14.2", "r": "1.1", "fill": "currentColor", "stroke": "none" }
            circle { "cx": "12.7", "cy": "7.5", "r": "1.1", "fill": "currentColor", "stroke": "none" }
        },
        IconKind::Fingerprint => rsx! {
            path { "d": "M12 21c-3-2-5-5-5-9a5 5 0 0 1 10 0v1.5" }
            path { "d": "M9 21c-1.4-1.7-2.3-4-2.3-6.5a2.6 2.6 0 0 1 5.2-.2c.1 1.2.1 2.2-.4 3.2" }
            path { "d": "M12 3.3c.7 0 1.4.1 2 .3M15.5 5c1.5 1.3 2.5 3.2 2.5 5.4v2" }
        },
        IconKind::Activity => rsx! {
            circle { "cx": "12", "cy": "12", "r": "9" }
            path { "d": "M5 12h3l1.8-4.5L13 16l1.8-4h4.2" }
        },
        IconKind::Money => rsx! {
            circle { "cx": "12", "cy": "12", "r": "8" }
            path { "d": "M12 7v10" }
            path { "d": "M9.3 9.6c0-1.1 1.2-2 2.7-2s2.7.9 2.7 2-1.2 1.6-2.7 2-2.7.9-2.7 2 1.2 2 2.7 2 2.7-.9 2.7-2" }
        },
        IconKind::Ballot => rsx! {
            rect { "x": "4", "y": "4", "width": "16", "height": "16", "rx": "2" }
            path { "d": "M8 12.5l2.6 2.6L16 9.5" }
        },
        IconKind::Lightbulb => rsx! {
            path { "d": "M9 18h6M10 21h4" }
            path { "d": "M12 3a6 6 0 0 0-3.5 10.9c.6.45 1 1.15 1 1.9v.2h5v-.2c0-.75.4-1.45 1-1.9A6 6 0 0 0 12 3z" }
        },
        IconKind::Trash => rsx! {
            path { "d": "M4 7h16M9 7V5a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v2" }
            path { "d": "M6 7l1 13a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2l1-13" }
            path { "d": "M10 11v6M14 11v6" }
        },
        IconKind::Cone => rsx! {
            path { "d": "M12 3l5 15H7z" }
            path { "d": "M9 12h6M8 15h8" }
            rect { "x": "3", "y": "18", "width": "18", "height": "3", "rx": "1" }
        },
        IconKind::XLogo => rsx! {
            path { "d": "M4 4l16 16M20 4L4 20" }
        },
        IconKind::Facebook => rsx! {
            circle { "cx": "12", "cy": "12", "r": "9" }
            path {
                "fill": "currentColor",
                "stroke": "none",
                "d": "M13.4 21v-6.6h2.2l.3-2.6h-2.5v-1.6c0-.7.2-1.2 1.3-1.2h1.3V6.6c-.2 0-1-.1-1.9-.1-1.9 0-3.2 1.2-3.2 3.3v1.9H8.7v2.6h2.2V21z",
            }
        },
        IconKind::Instagram => rsx! {
            rect { "x": "3", "y": "3", "width": "18", "height": "18", "rx": "5" }
            circle { "cx": "12", "cy": "12", "r": "4" }
            circle { "cx": "17.3", "cy": "6.7", "r": "1", "fill": "currentColor", "stroke": "none" }
        },
        IconKind::LinkedIn => rsx! {
            rect { "x": "3", "y": "3", "width": "18", "height": "18", "rx": "2" }
            circle { "cx": "7.2", "cy": "8.2", "r": "1.2", "fill": "currentColor", "stroke": "none" }
            path { "d": "M7.2 11v7" }
            path { "d": "M12 18v-4.5c0-1.4 1.1-2.5 2.5-2.5s2.5 1.1 2.5 2.5V18M12 11v.5" }
        },
        IconKind::YouTube => rsx! {
            rect { "x": "2", "y": "5", "width": "20", "height": "14", "rx": "4" }
            path { "fill": "currentColor", "stroke": "none", "d": "M10 9l6 3-6 3z" }
        },
    }
}
