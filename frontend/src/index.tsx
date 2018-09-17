import * as ReactDom from 'react-dom';
import * as React from 'react';
import { applyMiddleware, createStore } from 'redux';
import { Provider } from "react-redux";
import { ConnectedRouter, connectRouter, routerMiddleware } from 'connected-react-router'
import createBrowserHistory from "history/createBrowserHistory";
import { Route, Switch } from "react-router";
import { BROWSER_URL_BASENAME } from "./constants/buildConstants";
import { handleGithubPagesSpaRedirect } from "./github-pages/gh-pages-spa-redirect-handler";
import { Frontpage } from "./frontpage/frontpage";

handleGithubPagesSpaRedirect();

const reducer = (state:  {}) => state;

const history = createBrowserHistory({
    basename: BROWSER_URL_BASENAME
});

const store = createStore(
    connectRouter(history)(reducer),
    {},
    applyMiddleware(
        routerMiddleware(history)
    )
);

const appContainer = document.createElement('div');
document.body.appendChild(appContainer)

ReactDom.render(
    <Provider store={store}>
        <ConnectedRouter history={history}>
            <Switch>
                <Route exact path="/" render={() => <Frontpage />}/>
                <Route render={() => (<div>where?</div>)} />
            </Switch>
        </ConnectedRouter>
    </Provider>,
    appContainer);
