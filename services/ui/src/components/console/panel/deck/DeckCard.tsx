import { createMemo, Match, Switch } from "solid-js";
import { Card } from "../../config/types";
import FieldCard from "./FieldCard";

const DeckCard = (props) => {
  return (
    <Switch
      fallback={
        <FieldCard card={props.card} value={props.data?.[props.card?.key]} />
      }
    >
      <Match when={props.card?.kind === Card.TABLE}>
        <div>Table Card</div>
      </Match>
    </Switch>
  );
};

export default DeckCard;
