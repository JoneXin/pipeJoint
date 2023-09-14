import ProxyTable from "./views/proxyTable";
import logo from "./assets/logo.png";

function App() {
    return (
        <div>
            <div
                className="header"
                style={{
                    display: "flex",
                    alignItems: "center",
                    justifyContent: "center",
                }}
            >
                <h1 style={{ fontFamily: "sans-serif", color: "green" }}>
                    Pipe Joint
                </h1>
                <img src={logo} alt="" height={150} width={150} />
            </div>
            <ProxyTable></ProxyTable>
        </div>
    );
}

export default App;
