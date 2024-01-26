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


import * as runtime from '../runtime';
import type {
  ApiErrorResponse,
  CreateFindingDefinitionRequest,
  FullFindingDefinition,
  ListFindingDefinitions,
  UuidResponse,
} from '../models';
import {
    ApiErrorResponseFromJSON,
    ApiErrorResponseToJSON,
    CreateFindingDefinitionRequestFromJSON,
    CreateFindingDefinitionRequestToJSON,
    FullFindingDefinitionFromJSON,
    FullFindingDefinitionToJSON,
    ListFindingDefinitionsFromJSON,
    ListFindingDefinitionsToJSON,
    UuidResponseFromJSON,
    UuidResponseToJSON,
} from '../models';

export interface CreateFindingDefinitionOperationRequest {
    createFindingDefinitionRequest: CreateFindingDefinitionRequest;
}

export interface GetFindingDefinitionRequest {
    uuid: string;
}

/**
 * 
 */
export class KnowledgeBaseApi extends runtime.BaseAPI {

    /**
     * Add a definition for a finding  These definition serve as reference and knowledge base in kraken. They can be used to create a finding that references a definition and links it to one or multiple aggregations.
     * Add a definition for a finding
     */
    async createFindingDefinitionRaw(requestParameters: CreateFindingDefinitionOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<UuidResponse>> {
        if (requestParameters.createFindingDefinitionRequest === null || requestParameters.createFindingDefinitionRequest === undefined) {
            throw new runtime.RequiredError('createFindingDefinitionRequest','Required parameter requestParameters.createFindingDefinitionRequest was null or undefined when calling createFindingDefinition.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        headerParameters['Content-Type'] = 'application/json';

        const response = await this.request({
            path: `/api/v1/findingDefinitions`,
            method: 'POST',
            headers: headerParameters,
            query: queryParameters,
            body: CreateFindingDefinitionRequestToJSON(requestParameters.createFindingDefinitionRequest),
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => UuidResponseFromJSON(jsonValue));
    }

    /**
     * Add a definition for a finding  These definition serve as reference and knowledge base in kraken. They can be used to create a finding that references a definition and links it to one or multiple aggregations.
     * Add a definition for a finding
     */
    async createFindingDefinition(requestParameters: CreateFindingDefinitionOperationRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<UuidResponse> {
        const response = await this.createFindingDefinitionRaw(requestParameters, initOverrides);
        return await response.value();
    }

    /**
     * Retrieve all finding definitions
     * Retrieve all finding definitions
     */
    async getAllFindingDefinitionsRaw(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<ListFindingDefinitions>> {
        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/findingDefinitions`,
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => ListFindingDefinitionsFromJSON(jsonValue));
    }

    /**
     * Retrieve all finding definitions
     * Retrieve all finding definitions
     */
    async getAllFindingDefinitions(initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<ListFindingDefinitions> {
        const response = await this.getAllFindingDefinitionsRaw(initOverrides);
        return await response.value();
    }

    /**
     * Retrieve a specific finding definition
     * Retrieve a specific finding definition
     */
    async getFindingDefinitionRaw(requestParameters: GetFindingDefinitionRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<runtime.ApiResponse<FullFindingDefinition>> {
        if (requestParameters.uuid === null || requestParameters.uuid === undefined) {
            throw new runtime.RequiredError('uuid','Required parameter requestParameters.uuid was null or undefined when calling getFindingDefinition.');
        }

        const queryParameters: any = {};

        const headerParameters: runtime.HTTPHeaders = {};

        const response = await this.request({
            path: `/api/v1/findingDefinitions/{uuid}`.replace(`{${"uuid"}}`, encodeURIComponent(String(requestParameters.uuid))),
            method: 'GET',
            headers: headerParameters,
            query: queryParameters,
        }, initOverrides);

        return new runtime.JSONApiResponse(response, (jsonValue) => FullFindingDefinitionFromJSON(jsonValue));
    }

    /**
     * Retrieve a specific finding definition
     * Retrieve a specific finding definition
     */
    async getFindingDefinition(requestParameters: GetFindingDefinitionRequest, initOverrides?: RequestInit | runtime.InitOverrideFunction): Promise<FullFindingDefinition> {
        const response = await this.getFindingDefinitionRaw(requestParameters, initOverrides);
        return await response.value();
    }

}
