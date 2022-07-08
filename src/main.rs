use yew::prelude::*;

#[function_component(Root)]
fn root() -> Html {
    html! {
        <>
            <h1>{"SQL Project"}</h1>
            <div>
                <label for="first">{"First Name:"}</label>
                <input type="text" id="first" name="first" />

                <label for="last">{"Last Name:"}</label>
                <input type="text" id="last" name="last" />
                
                <button type="button">{"Submit"}</button>
            </div>

            <div>
                <table>
                    <tr>
                        <th>{"id"}</th>
                        <th>{"first"}</th>
                        <th>{"last"}</th>
                    </tr>
                    <tr>
                        <td>{"1"}</td>
                        <td>{"skyler"}</td>
                        <td>{"favors"}</td>
                    </tr>
                </table>
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<Root>();
}
