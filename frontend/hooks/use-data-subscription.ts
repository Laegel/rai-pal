import { useSetAtom, atom } from "jotai";
import { AppEvent, EventPayload, useAppEvent } from "./use-app-event";

export function dataSubscription<
	TEvent extends AppEvent,
	TDefaultValue extends EventPayload<TEvent> | undefined,
>(event: TEvent, defaultValue?: TDefaultValue) {
	type Data = TDefaultValue extends undefined
		? EventPayload<TEvent> | undefined
		: EventPayload<TEvent>;

	const stateAtom = atom<Data>(defaultValue as Data);

	function useDataSubscription() {
		const setData = useSetAtom(stateAtom);

		useAppEvent(event, (payload) => setData(payload as Data));
	}

	return [stateAtom, useDataSubscription] as const;
}
