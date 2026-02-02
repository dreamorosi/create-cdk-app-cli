#!/usr/bin/env node
import 'source-map-support/register';
import { App } from 'aws-cdk-lib';
import { ReferenceProjectStack } from '../lib/referenceproject-stack.js';

const app = new App();
new ReferenceProjectStack(app, 'ReferenceProjectStack', {});