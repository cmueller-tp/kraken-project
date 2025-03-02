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
/**
 * A host was deleted
 * @export
 * @interface WsMessageOneOf19
 */
export interface WsMessageOneOf19 {
    /**
     * The workspace this host is related to
     * @type {string}
     * @memberof WsMessageOneOf19
     */
    workspace: string;
    /**
     * The uuid of the deleted host
     * @type {string}
     * @memberof WsMessageOneOf19
     */
    host: string;
    /**
     * 
     * @type {string}
     * @memberof WsMessageOneOf19
     */
    type: WsMessageOneOf19TypeEnum;
}


/**
 * @export
 */
export const WsMessageOneOf19TypeEnum = {
    DeletedHost: 'DeletedHost'
} as const;
export type WsMessageOneOf19TypeEnum = typeof WsMessageOneOf19TypeEnum[keyof typeof WsMessageOneOf19TypeEnum];


/**
 * Check if a given object implements the WsMessageOneOf19 interface.
 */
export function instanceOfWsMessageOneOf19(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "workspace" in value;
    isInstance = isInstance && "host" in value;
    isInstance = isInstance && "type" in value;

    return isInstance;
}

export function WsMessageOneOf19FromJSON(json: any): WsMessageOneOf19 {
    return WsMessageOneOf19FromJSONTyped(json, false);
}

export function WsMessageOneOf19FromJSONTyped(json: any, ignoreDiscriminator: boolean): WsMessageOneOf19 {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'workspace': json['workspace'],
        'host': json['host'],
        'type': json['type'],
    };
}

export function WsMessageOneOf19ToJSON(value?: WsMessageOneOf19 | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'workspace': value.workspace,
        'host': value.host,
        'type': value.type,
    };
}

