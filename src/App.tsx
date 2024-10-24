import "./tailwind-global.css";
import {useState} from "react";
import {invoke} from "@tauri-apps/api/core";

function App() {

    const [value, setValue] = useState("");

    async function getValueByKey(key: string) {
        setValue(await invoke("get_value_by_key", {key: key}));
    }

    return (
        <main>
            <div>
                <input onChange={async e => {
                    await getValueByKey(e.currentTarget.value);
                }} placeholder="请输入关键词"/>
                <div>{value}</div>
            </div>
        </main>
    );
}

export default App;
