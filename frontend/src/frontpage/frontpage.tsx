import * as React from "react";

export class Frontpage extends React.Component {
    public render() {
        return <div>
            <h1>Extreme Startup!</h1>
            <button onClick={() => alert("hi")}>Start a new game!</button>
        </div>;
    }
}