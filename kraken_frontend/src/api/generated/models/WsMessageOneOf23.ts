/* tslint:disable */
/* eslint-disable */
/**
 * kraken
 * The core component of kraken-project
 *
 * The version of the OpenAPI document: 0.1.0
 * Contact: git@omikron.dev
 *
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * https://openapi-generator.tech
 * Do not edit the class manually.
 */

import { exists, mapValues } from '../runtime';
import type { AggregationType } from './AggregationType';
import {
    AggregationTypeFromJSON,
    AggregationTypeFromJSONTyped,
    AggregationTypeToJSON,
} from './AggregationType';

/**
 * Workspace tags were updated on an aggregation
 * @export
 * @interface WsMessageOneOf23
 */
export interface WsMessageOneOf23 {
    /**
     * The workspace the aggregation is related to
     * @type {string}
     * @memberof WsMessageOneOf23
     */
    workspace: string;
    /**
     * 
     * @type {AggregationType}
     * @memberof WsMessageOneOf23
     */
    aggregation: AggregationType;
    /**
     * The uuid of the model
     * @type {string}
     * @memberof WsMessageOneOf23
     */
    uuid: string;
    /**
     * The updated list of tags
     * @type {Array<string>}
     * @memberof WsMessageOneOf23
     */
    tags: Array<string>;
    /**
     * 
     * @type {string}
     * @memberof WsMessageOneOf23
     */
    type: WsMessageOneOf23TypeEnum;
}


/**
 * @export
 */
export const WsMessageOneOf23TypeEnum = {
    UpdatedWorkspaceTags: 'UpdatedWorkspaceTags'
} as const;
export type WsMessageOneOf23TypeEnum = typeof WsMessageOneOf23TypeEnum[keyof typeof WsMessageOneOf23TypeEnum];


/**
 * Check if a given object implements the WsMessageOneOf23 interface.
 */
export function instanceOfWsMessageOneOf23(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "workspace" in value;
    isInstance = isInstance && "aggregation" in value;
    isInstance = isInstance && "uuid" in value;
    isInstance = isInstance && "tags" in value;
    isInstance = isInstance && "type" in value;

    return isInstance;
}

export function WsMessageOneOf23FromJSON(json: any): WsMessageOneOf23 {
    return WsMessageOneOf23FromJSONTyped(json, false);
}

export function WsMessageOneOf23FromJSONTyped(json: any, ignoreDiscriminator: boolean): WsMessageOneOf23 {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'workspace': json['workspace'],
        'aggregation': AggregationTypeFromJSON(json['aggregation']),
        'uuid': json['uuid'],
        'tags': json['tags'],
        'type': json['type'],
    };
}

export function WsMessageOneOf23ToJSON(value?: WsMessageOneOf23 | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'workspace': value.workspace,
        'aggregation': AggregationTypeToJSON(value.aggregation),
        'uuid': value.uuid,
        'tags': value.tags,
        'type': value.type,
    };
}

