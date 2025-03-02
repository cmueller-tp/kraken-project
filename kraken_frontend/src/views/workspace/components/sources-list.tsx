import { SimpleAggregationSource } from "../../../api/generated";
import React from "react";
import { ObjectFns } from "../../../utils/helper";
import Bubble from "../../../components/bubble";
import { AttackResolver, ATTACKS } from "../../../utils/attack-resolver";

type SourcesListProps = {
    sources: SimpleAggregationSource;
};

const { Undefined, ...SOURCE_LIST_ATTACKS } = ATTACKS;
const SOURCE_LIST_TYPES: Omit<AttackResolver, "Undefined"> = SOURCE_LIST_ATTACKS;

export default function SourcesList(props: SourcesListProps) {
    return (
        <div className={"bubble-list"}>
            {ObjectFns.entries(SOURCE_LIST_TYPES).map(([_, { key, abbreviation, long }]) => {
                // @ts-ignore
                const count = props.sources[key];
                return count > 0 ? <Bubble color={"primary"} name={`${abbreviation} ${count}`} title={long} /> : null;
            })}
            {props.sources.manual ? <Bubble name={"MI"} title={"Manually inserted"} /> : null}
        </div>
    );
}
