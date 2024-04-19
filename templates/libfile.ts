import { Stack, type StackProps, CfnOutput } from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { Runtime } from 'aws-cdk-lib/aws-lambda';
import { NodejsFunction, OutputFormat } from 'aws-cdk-lib/aws-lambda-nodejs';

export class pascalcase-nameStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const fn = new NodejsFunction(this, 'MyFunction', {
      runtime: Runtime.NODEJS_20_X,
      entry: '../src/index.ts',
      handler: 'handler',
      bundling: {
        minify: true,
        mainFields: ['module', 'main'],
        sourceMap: true,
        format: OutputFormat.ESM,
      }
    });

    new CfnOutput(this, 'FunctionArn', {
      value: fn.functionArn,
    });
  }
}