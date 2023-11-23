import {
    createContext,
    useContext,
    createEffect,
    JSXElement
} from 'solid-js';
import { createStore, SetStoreFunction, Store } from 'solid-js/store';
import { rootInitialState, RootState } from './_state';

const StoreContext = createContext();

const createProvidedStore = () =>
    createStore<RootState>(rootInitialState as unknown as RootState);

type StoreProviderProps = {
    children: JSXElement;
    store?: Store<RootState>;
    setStore?: SetStoreFunction<RootState>;
};

export const StoreProvider = (props: StoreProviderProps) => {
    const store = createProvidedStore();

    createEffect(() => { 
        
    });

    return (
        <StoreContext.Provider value={store}>
            {props.children}
        </StoreContext.Provider>
    );
};

export const useStore = () =>
    useContext(StoreContext) as [Store<RootState>, SetStoreFunction<RootState>];
