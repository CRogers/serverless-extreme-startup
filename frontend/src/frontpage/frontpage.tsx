import * as React from "react";
import { MouseEvent } from "react";
import { API_URL_BASE } from "../constants/buildConstants";

interface FrontpageState {
    gameId: String;
    gameData: String;
}

export class Frontpage extends React.Component<{}, FrontpageState> {
    public state: FrontpageState = {
        gameId: 'foo',
        gameData: ''
    };

    public render() {
        return <div>
            <h1>Extreme Startup!</h1>
            <label htmlFor="game_id">Game Name:</label>
            <input type="text" name="game_id" onChange={ev => this.updateGameId(ev.target.value)} />
            <br/>
            <input type="button" value="Start a new game!" onClick={ev => this.createGame(ev)} />
            <input type="button" value="Get game data" onClick={ev => this.getGame(ev)} />
            <pre>{this.state.gameData}</pre>
        </div>;
    }

    private updateGameId(gameId: String) {
        this.setState(prevState => ({ ...prevState, gameId }));
    }

    private createGame(event: MouseEvent<HTMLInputElement>) {
        event.preventDefault();
        fetch(API_URL_BASE + "create/" + this.state.gameId, {
            method: 'POST'
        });
    }

    private getGame(ev: React.MouseEvent) {
        ev.preventDefault();
        fetch(API_URL_BASE + "games/" + this.state.gameId)
            .then(response => response.text())
            .then(gameData => this.setState(prevState => ({ ...prevState, gameData })))
            .catch(reason => console.error(reason));
    }
}