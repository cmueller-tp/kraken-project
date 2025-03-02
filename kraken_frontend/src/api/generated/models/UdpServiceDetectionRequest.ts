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
import type { PortOrRange } from './PortOrRange';
import {
    PortOrRangeFromJSON,
    PortOrRangeFromJSONTyped,
    PortOrRangeToJSON,
} from './PortOrRange';

/**
 * The request to start a service detection
 * @export
 * @interface UdpServiceDetectionRequest
 */
export interface UdpServiceDetectionRequest {
    /**
     * The leech to use
     * 
     * Leave empty to use a random leech
     * @type {string}
     * @memberof UdpServiceDetectionRequest
     */
    leechUuid?: string | null;
    /**
     * The ip address the service listens on
     * @type {string}
     * @memberof UdpServiceDetectionRequest
     */
    address: string;
    /**
     * List of single ports and port ranges
     * 
     * If no values are supplied, 1-65535 is used as default
     * @type {Array<PortOrRange>}
     * @memberof UdpServiceDetectionRequest
     */
    ports?: Array<PortOrRange>;
    /**
     * The interval that should be wait between retries on a port.
     * 
     * The interval is specified in milliseconds.
     * @type {number}
     * @memberof UdpServiceDetectionRequest
     */
    retryInterval: number;
    /**
     * The number of times the connection should be retried if it failed.
     * @type {number}
     * @memberof UdpServiceDetectionRequest
     */
    maxRetries: number;
    /**
     * The time to wait until a connection is considered failed.
     * 
     * The timeout is specified in milliseconds.
     * @type {number}
     * @memberof UdpServiceDetectionRequest
     */
    timeout: number;
    /**
     * The concurrent task limit
     * @type {number}
     * @memberof UdpServiceDetectionRequest
     */
    concurrentLimit: number;
    /**
     * The workspace to execute the attack in
     * @type {string}
     * @memberof UdpServiceDetectionRequest
     */
    workspaceUuid: string;
}

/**
 * Check if a given object implements the UdpServiceDetectionRequest interface.
 */
export function instanceOfUdpServiceDetectionRequest(value: object): boolean {
    let isInstance = true;
    isInstance = isInstance && "address" in value;
    isInstance = isInstance && "retryInterval" in value;
    isInstance = isInstance && "maxRetries" in value;
    isInstance = isInstance && "timeout" in value;
    isInstance = isInstance && "concurrentLimit" in value;
    isInstance = isInstance && "workspaceUuid" in value;

    return isInstance;
}

export function UdpServiceDetectionRequestFromJSON(json: any): UdpServiceDetectionRequest {
    return UdpServiceDetectionRequestFromJSONTyped(json, false);
}

export function UdpServiceDetectionRequestFromJSONTyped(json: any, ignoreDiscriminator: boolean): UdpServiceDetectionRequest {
    if ((json === undefined) || (json === null)) {
        return json;
    }
    return {
        
        'leechUuid': !exists(json, 'leech_uuid') ? undefined : json['leech_uuid'],
        'address': json['address'],
        'ports': !exists(json, 'ports') ? undefined : ((json['ports'] as Array<any>).map(PortOrRangeFromJSON)),
        'retryInterval': json['retry_interval'],
        'maxRetries': json['max_retries'],
        'timeout': json['timeout'],
        'concurrentLimit': json['concurrent_limit'],
        'workspaceUuid': json['workspace_uuid'],
    };
}

export function UdpServiceDetectionRequestToJSON(value?: UdpServiceDetectionRequest | null): any {
    if (value === undefined) {
        return undefined;
    }
    if (value === null) {
        return null;
    }
    return {
        
        'leech_uuid': value.leechUuid,
        'address': value.address,
        'ports': value.ports === undefined ? undefined : ((value.ports as Array<any>).map(PortOrRangeToJSON)),
        'retry_interval': value.retryInterval,
        'max_retries': value.maxRetries,
        'timeout': value.timeout,
        'concurrent_limit': value.concurrentLimit,
        'workspace_uuid': value.workspaceUuid,
    };
}

