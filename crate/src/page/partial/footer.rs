use crate::{generated::css_classes::C, image_src, Msg, MAIL_TO_KAVIK};
use seed::{prelude::*, *};

pub fn view() -> impl View<Msg> {
    footer![
        class![
            C.h_16,
            C.shadow_2xl_above,
            C.flex,
            C.justify_center,
            // sm__
            C.sm__h_24,
        ],
        div![
            class![
                C.w_xs,
                C.h_full,
                C.px_5,
                C.flex,
                C.justify_between,
                C.items_center,
                // sm__
                C.sm__w_132
            ],
            div![
                class![
                    // lg__
                    C.lg__pb_3,
                ],
                img![
                    class![
                        C.inline,
                        C.w_6,
                        C.align_baseline,
                        // sm__
                        C.sm__w_12
                    ],
                    attrs! {
                        At::Src => image_src("logo.svg")
                    }
                ],
                span![
                    class![
                        C.ml_1,
                        C.font_display,
                        C.font_semibold,
                        C.text_15,
                        C.text_yellow_6,
                        // sm__
                        C.sm__mt_2,
                        C.sm__text_25,
                    ],
                    "2019"
                ]
            ],
            a![
                attrs! {
                    At::Href => MAIL_TO_KAVIK,
                },
                class![
                    C.font_display,
                    C.font_semibold,
                    C.text_16,
                    C.text_gray_10,
                    C.underline,
                    C.underline_yellow_7,
                    // sm__
                    C.sm__text_26
                ],
                "martin@kavik.cz"
            ],
            div![
                class![C.cursor_pointer, C.h_full, C.flex, C.items_center,],
                ev(Ev::Click, |_| Msg::ScrollToTop),
                img![
                    class![
                        C.mt_1, C.w_12, // sm__
                        C.sm__w_16
                    ],
                    attrs! {
                        At::Src => image_src("top.svg")
                    }
                ],
            ]
        ]
    ]
}
