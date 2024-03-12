use leptonic::prelude as ltn;
use leptos::*;

#[component]
pub fn DropdownMenu(title: String,
                    #[prop(into)] show_when: RwSignal<bool>,
                    children: Children) -> impl IntoView {
    #[derive(Clone)] struct Title (String);
    #[derive(Clone)] struct Items (Fragment);
    provide_context(Title(title));
    provide_context(Items(children()));

    view! {
        <ltn::Modal class="menu" show_when on_escape=move || show_when.set(false) >
            <ltn::ModalHeader><ltn::ModalTitle>
                {
                    let title = use_context::<Title>()
                        .expect("should find my title");
                    title.0
                }
            </ltn::ModalTitle></ltn::ModalHeader>
            <ltn::ModalBody>
                <ltn::TableContainer>
                    <ltn::Table bordered=true hoverable=true>
                        <ltn::TableBody>
                            { ||
                             use_context::<Items>()
                             .expect("should find my items")
                             .0.nodes.into_iter()
                             .map(|child| view! { <ltn::TableRow>
                                                      <ltn::TableCell>{child}</ltn::TableCell>
                                                  </ltn::TableRow> })
                             .collect_view()
                            }
                        </ltn::TableBody>
                    </ltn::Table>
                </ltn::TableContainer>
            </ltn::ModalBody>
        </ltn::Modal>
    }
}
