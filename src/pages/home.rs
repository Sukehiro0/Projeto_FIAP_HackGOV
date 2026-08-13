use dioxus::prelude::*;

use crate::components::{Assistant, Categories, Hero, Security, Services, Stats, StatusPanel};

#[component]
pub fn Home() -> Element {
    // Ativa o fade-in das seções ao entrarem na viewport (IntersectionObserver)
    use_effect(|| {
        document::eval(
            r#"
            setTimeout(() => {
                const obs = new IntersectionObserver((entries) => {
                    entries.forEach((entry) => {
                        if (entry.isIntersecting) {
                            entry.target.classList.add('is-visible');
                            obs.unobserve(entry.target);
                        }
                    });
                }, { threshold: 0.15 });
                document.querySelectorAll('.reveal').forEach((el) => obs.observe(el));
            }, 50);
            "#,
        );
    });

    rsx! {
        Hero {}
        Assistant {}
        Stats {}
        Services {}
        Categories {}
        Security {}
        StatusPanel {}
    }
}
