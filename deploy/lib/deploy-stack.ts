import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { Code, Function, Runtime, FunctionUrlAuthType } from "aws-cdk-lib/aws-lambda";
import { CfnOutput } from "aws-cdk-lib";
import * as path from 'path';
import * as dynamodb from 'aws-cdk-lib/aws-dynamodb';
import * as iam from 'aws-cdk-lib/aws-iam';

export class DeployStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const table = new dynamodb.Table(this, 'BooksTable', {
      tableName: 'booksTable',
      partitionKey: { name: 'PK', type: dynamodb.AttributeType.STRING },
      sortKey: { name: 'SK', type: dynamodb.AttributeType.STRING },
      billingMode: dynamodb.BillingMode.PROVISIONED,
    });

    // Create the IAM role for the Lambda function
    const lambdaRole = new iam.Role(this, 'LambdaRole', {
      assumedBy: new iam.ServicePrincipal('lambda.amazonaws.com'),
    });

    // Attach the CloudWatch Logs policy to the role
    lambdaRole.addToPolicy(new iam.PolicyStatement({
      actions: [
        'logs:CreateLogGroup',
        'logs:CreateLogStream',
        'logs:PutLogEvents',
      ],
      resources: ['*'],
    }));

    table.grantFullAccess(lambdaRole);

    const handler = new Function(this, "BooksApi", {
      code: Code.fromAsset(path.join(__dirname, "..", "..", "target/lambda/books-api-lambda-rust-based")),
      runtime: Runtime.PROVIDED_AL2,
      handler: "does_not_matter",
      functionName: "book-api-function",
      role: lambdaRole
    });

    const fnUrl = handler.addFunctionUrl({
      authType: FunctionUrlAuthType.NONE,
    });
    
    const generatedUrl = new CfnOutput(this, 'TheUrl', {
      value: fnUrl.url,
    });

    console.log(generatedUrl.value);
  }
}
