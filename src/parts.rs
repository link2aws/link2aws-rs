/// Provides methods to build an AWS console link or rebuild the ARN
/// for any struct that has the required getters for the ARN parts.
///
/// # Example using [`Arn`](`crate::Arn`)
///
/// ```
/// use link2aws::{Arn, ArnParts};
///
/// // Arn implements ArnParts.
/// let arn = Arn::new("arn:aws:s3:::abc123").unwrap();
/// let expected_link = "https://s3.console.aws.amazon.com/s3/buckets/abc123";
/// assert_eq!(arn.link().unwrap(), expected_link);
/// ```
///
/// # How to use with your own struct
///
/// ```
/// use link2aws::ArnParts;
///
/// struct MyArnParts {
/// }
///
/// impl<'a> ArnParts<'a> for MyArnParts {
///   fn partition(&self) -> &str { "aws" }
///   fn service(&self) -> &str { "s3" }
///   fn region(&self) -> &str { "" }
///   fn account(&self) -> &str { "" }
///   fn resource_revision(&self) -> &str { "" }
///   fn resource_type(&self) -> &str { "" }
///   fn resource_id(&self) -> &str { "abc123" }
///   fn has_path(&self) -> bool { false }
/// }
///
/// let my_arn = MyArnParts {};
/// let expected_link = "https://s3.console.aws.amazon.com/s3/buckets/abc123";
/// assert_eq!(my_arn.link().unwrap(), expected_link);
/// ```
#[allow(private_bounds)] // `ArnPartsHelper` is private intentionally.
pub trait ArnParts<'a>: ArnPartsHelper<'a> {
    // Accessors for the individual parts of the ARN.
    fn partition(&self) -> &str;
    fn service(&self) -> &str;
    fn region(&self) -> &str;
    fn account(&self) -> &str;
    fn resource_revision(&self) -> &str;
    fn resource_type(&self) -> &str;
    fn resource_id(&self) -> &str;
    fn has_path(&self) -> bool;

    /// Convert the ARN parts back into an ARN.
    fn build(&self) -> String {
        let partition = self.partition();
        let service = self.service();
        let region = self.region();
        let account = self.account();
        let resource_type = self.resource_type();
        let has_path = self.has_path();
        let id = self.resource_id();
        let revision = self.resource_revision();

        let colon_before_type = if !resource_type.is_empty() { ":" } else { "" };
        let slash_before_type = if service == "apigateway" { "/" } else { "" };
        let delim_before_id = if has_path { "/" } else { ":" };
        let colon_before_revision = if !revision.is_empty() { ":" } else { "" };

        #[rustfmt::skip]
        let arn = [
            "arn",
            ":", partition,
            ":", service,
            ":", region,
            ":", account,
            colon_before_type, slash_before_type, resource_type,
            delim_before_id, id,
            colon_before_revision, revision,
        ].concat();

        arn
    }

    /// Returns a link to the AWS console for this ARN.
    ///
    /// Returns None if we don't have a link for this ARN.
    /// This does **not** mean that the ARN itself is invalid.
    fn link(&self) -> Option<String> {
        match (self.service(), self.resource_type()) {
            // Alexa for Business
            // ("a4b", "addressbook") => None,
            // ("a4b", "conferenceprovider") => None,
            // ("a4b", "contact") => None,
            // ("a4b", "device") => None,
            // ("a4b", "networkprofile") => None,
            // ("a4b", "profile") => None,
            // ("a4b", "room") => None,
            // ("a4b", "schedule") => None,
            // ("a4b", "skillgroup") => None,
            // ("a4b", "user") => None,

            // IAM Access Analyzer
            ("access-analyzer", "analyzer") => Some(format!(
                "https://{region}.{domain}/access-analyzer/home?region={region}#/analyzer/{resource}",
                region = self.region(),
                domain = self.domain()?,
                resource = self.resource_id(),
            )),

            // Amazon EC2
            ("acm", "certificate") => Some(format!(
                "https://{domain}/acm/home?region={region}#/?id={resource}",
                domain = self.domain()?,
                region = self.region(),
                resource = self.resource_id(),
            )),

            // AWS Certificate Manager Private Certificate Authority
            // ("acm-pca", "certificate-authority") => None,

            // Amazon Managed Workflows for Apache Airflow
            // ("airflow", "environment") => None,
            // ("airflow", "rbac-role") => None,

            // AWS Amplify
            ("amplify", "apps") => {
                if self.resource_id().contains("/jobs/") {
                    let resource_split: Vec<&str> = self.resource_id().split('/').collect();
                    if resource_split.len() >= 4 {
                        let app_id = resource_split[0];
                        let branch = resource_split[2];
                        let mut job = resource_split[resource_split.len() - 1];
                        // Remove leading zeros from job
                        job = job.trim_start_matches('0');
                        return Some(format!(
                            "https://{region}.{domain}/amplify/home?region={region}#/{app_id}/{branch}/{job}",
                            region = self.region(),
                            domain = self.domain()?,
                            app_id = app_id,
                            branch = branch,
                            job = job,
                        ));
                    }
                }
                None
            }
            // ("amplify", "branches") => None,
            // ("amplify", "domains") => None,
            // ("amplify", "jobs") => None,

            // AWS Amplify Admin
            // ("amplifybackend", "api") => None,
            // ("amplifybackend", "auth") => None,
            // ("amplifybackend", "backend") => None,
            // ("amplifybackend", "config") => None,
            // ("amplifybackend", "environment") => None,
            // ("amplifybackend", "job") => None,
            // ("amplifybackend", "token") => None,

            // Amazon API Gateway
            ("apigateway", "restapis") => Some(format!(
                "https://{region}.{domain}/apigateway/main/apis/{resource}/resources?api={resource}&region={region}",
                region = self.region(),
                domain = self.domain()?,
                resource = self.resource_id(),
            )),

            // Amazon AppIntegrations
            // ("app-integrations", "event-integration") => None,
            // ("app-integrations", "event-integration-association") => None,

            // AWS AppConfig
            // ("appconfig", "application") => None,
            // ("appconfig", "configurationprofile") => None,
            // ("appconfig", "deployment") => None,
            // ("appconfig", "deploymentstrategy") => None,
            // ("appconfig", "environment") => None,
            // ("appconfig", "hostedconfigurationversion") => None,

            // Amazon AppFlow
            // ("appflow", "connectorprofile") => None,
            // ("appflow", "flow") => None,

            // AWS App Mesh
            // ("appmesh", "gatewayRoute") => None,
            // ("appmesh", "mesh") => None,
            // ("appmesh", "route") => None,
            // ("appmesh", "virtualGateway") => None,
            // ("appmesh", "virtualNode") => None,
            // ("appmesh", "virtualRouter") => None,
            // ("appmesh", "virtualService") => None,

            // AWS App Mesh Preview
            // ("appmesh-preview", "gatewayRoute") => None,
            // ("appmesh-preview", "mesh") => None,
            // ("appmesh-preview", "route") => None,
            // ("appmesh-preview", "virtualGateway") => None,
            // ("appmesh-preview", "virtualNode") => None,
            // ("appmesh-preview", "virtualRouter") => None,
            // ("appmesh-preview", "virtualService") => None,

            // Amazon AppStream 2.0
            // ("appstream", "fleet") => None,
            // ("appstream", "image") => None,
            // ("appstream", "image-builder") => None,
            // ("appstream", "stack") => None,

            // AWS AppSync
            // ("appsync", "appsync") => None,
            // ("appsync", "datasource") => None,
            // ("appsync", "field") => None,
            // ("appsync", "function") => None,
            // ("appsync", "graphqlapi") => None,
            // ("appsync", "type") => None,

            // Amazon Managed Service for Prometheus
            // ("aps", "workspace") => None,

            // AWS Artifact
            // ("artifact", "agreement") => None,
            // ("artifact", "customer-agreement") => None,
            // ("artifact", "report-package") => None,

            // Amazon Athena
            // ("athena", "datacatalog") => None,
            // ("athena", "workgroup") => None,

            // AWS Audit Manager
            // ("auditmanager", "assessment") => None,
            // ("auditmanager", "assessmentControlSet") => None,
            // ("auditmanager", "assessmentFramework") => None,
            // ("auditmanager", "control") => None,

            // Amazon EC2 Auto Scaling
            ("autoscaling", "autoScalingGroup") => Some(format!(
                "https://{region}.{domain}/ec2/home?region={region}#AutoScalingGroupDetails:id={group_name};view=details",
                region = self.region(),
                domain = self.domain()?,
                group_name = self.resource_id().split_once('/').unwrap_or_default().1,
            )),
            // ("autoscaling", "launchConfiguration") => None,
            // ("autoscaling", "lifecycleHook") => None,
            // ("autoscaling", "scalingPolicy") => None,
            // ("autoscaling", "scheduledAction") => None,

            // AWS Marketplace Catalog
            // ("aws-marketplace", "ChangeSet") => None,
            // ("aws-marketplace", "Entity") => None,

            // AWS Backup
            // ("backup", "backup-plan") => None,
            ("backup", "backup-vault") => Some(format!(
                "https://{domain}/backup/home?region={region}#/backupvaults/details/{resource}",
                domain = self.domain()?,
                region = self.region(),
                resource = self.resource_id(),
            )),

            // AWS Batch
            // ("batch", "compute-environment") => None,
            // ("batch", "job") => None,
            // ("batch", "job-definition") => None,
            // ("batch", "job-queue") => None,

            // Amazon Braket
            // ("braket", "quantum-task") => None,

            // AWS Budget Service
            // ("budgets", "budget") => None,
            // ("budgets", "budgetAction") => None,

            // Amazon Keyspaces (for Apache Cassandra)
            // ("cassandra", "keyspace") => None,
            // ("cassandra", "table") => None,

            // AWS Service Catalog
            // ("catalog", "Portfolio") => None,
            // ("catalog", "Product") => None,

            // AWS Chatbot
            // ("chatbot", "ChatbotConfiguration") => None,

            // Amazon Chime
            // ("chime", "app-instance") => None,
            // ("chime", "app-instance-user") => None,
            // ("chime", "channel") => None,
            // ("chime", "meeting") => None,

            // AWS Cloud9
            // ("cloud9", "environment") => None,

            // Amazon Cloud Directory
            // ("clouddirectory", "appliedSchema") => None,
            // ("clouddirectory", "developmentSchema") => None,
            // ("clouddirectory", "directory") => None,
            // ("clouddirectory", "publishedSchema") => None,

            // AWS CloudFormation
            // ("cloudformation", "changeset") => None,
            // ("cloudformation", "stack") => None,
            // ("cloudformation", "stackset") => None,
            // ("cloudformation", "stackset-target") => None,
            // ("cloudformation", "type") => None,

            // Amazon CloudFront
            // ("cloudfront", "cache-policy") => None,
            ("cloudfront", "distribution") => Some(format!(
                "https://{domain}/cloudfront/v4/home#/distributions/{resource}",
                domain = self.domain()?,
                resource = self.resource_id(),
            )),
            // ("cloudfront", "field-level-encryption") => None,
            // ("cloudfront", "field-level-encryption-profile") => None,
            // ("cloudfront", "origin-access-identity") => None,
            // ("cloudfront", "origin-request-policy") => None,
            // ("cloudfront", "streaming-distribution") => None,

            // AWS CloudHSM
            // ("cloudhsm", "backup") => None,
            // ("cloudhsm", "cluster") => None,

            // Amazon CloudSearch
            // ("cloudsearch", "domain") => None,

            // AWS CloudShell
            // ("cloudshell", "Environment") => None,

            // AWS CloudTrail
            // ("cloudtrail", "trail") => None,

            // Amazon CloudWatch
            // ("cloudwatch", "alarm") => None,
            // ("cloudwatch", "dashboard") => None,
            // ("cloudwatch", "insight-rule") => None,

            // AWS CodeArtifact
            // ("codeartifact", "domain") => None,
            // ("codeartifact", "package") => None,
            // ("codeartifact", "repository") => None,

            // AWS CodeBuild
            // ("codebuild", "build") => None,
            // ("codebuild", "build-batch") => None,
            // ("codebuild", "project") => None,
            // ("codebuild", "report") => None,
            // ("codebuild", "report-group") => None,

            // Amazon CodeGuru Reviewer
            // ("codecommit", "repository") => None,

            // AWS CodeConnections
            ("codeconnections", "connection") => Some(format!(
                "https://{region}.{domain}/codesuite/settings/{account}/{region}/{service}/{resource_type}s/{resource}",
                region = self.region(),
                domain = self.domain()?,
                account = self.account(),
                service = self.service(),
                resource_type = self.resource_type(),
                resource = self.resource_id(),
            )),

            // AWS CodeDeploy
            // ("codedeploy", "application") => None,
            // ("codedeploy", "deploymentconfig") => None,
            // ("codedeploy", "deploymentgroup") => None,
            // ("codedeploy", "instance") => None,

            // Amazon CodeGuru Profiler
            // ("codeguru-profiler", "ProfilingGroup") => None,

            // Amazon CodeGuru Reviewer
            // ("codeguru-reviewer", "association") => None,
            // ("codeguru-reviewer", "codereview") => None,

            // AWS CodePipeline
            ("codepipeline", "") => Some(format!(
                "https://{region}.{domain}/codesuite/codepipeline/pipelines/{resource}/view?region={region}",
                region = self.region(),
                domain = self.domain()?,
                resource = self.resource_id(),
            )),
            // ("codepipeline", "action") => None,
            // ("codepipeline", "actiontype") => None,
            // ("codepipeline", "pipeline") => None,
            // ("codepipeline", "stage") => None,
            // ("codepipeline", "webhook") => None,

            // AWS CodeStar
            // ("codestar", "project") => None,

            // AWS CodeStar Connections
            ("codestar-connections", "connection") => Some(format!(
                "https://{region}.{domain}/codesuite/settings/{account}/{region}/{service}/{resource_type}s/{resource}",
                region = self.region(),
                domain = self.domain()?,
                account = self.account(),
                service = self.service(),
                resource_type = self.resource_type(),
                resource = self.resource_id(),
            )),
            // ("codestar-connections", "host") => None,

            // AWS CodeStar Notifications
            // ("codestar-notifications", "notificationrule") => None,

            // Amazon Cognito Identity
            // ("cognito-identity", "identitypool") => None,

            // Amazon Cognito User Pools
            // ("cognito-idp", "userpool") => None,

            // Amazon Cognito Sync
            // ("cognito-sync", "dataset") => None,
            // ("cognito-sync", "identity") => None,
            // ("cognito-sync", "identitypool") => None,

            // Amazon Comprehend
            // ("comprehend", "document-classifier") => None,
            // ("comprehend", "document-classifier-endpoint") => None,
            // ("comprehend", "entity-recognizer") => None,
            // ("comprehend", "entity-recognizer-endpoint") => None,

            // AWS Config
            // ("config", "AggregationAuthorization") => None,
            // ("config", "ConfigRule") => None,
            // ("config", "ConfigurationAggregator") => None,
            // ("config", "ConformancePack") => None,
            // ("config", "OrganizationConfigRule") => None,
            // ("config", "OrganizationConformancePack") => None,
            // ("config", "RemediationConfiguration") => None,
            // ("config", "StoredQuery") => None,

            // Amazon Connect
            // ("connect", "contact") => None,
            // ("connect", "contact-flow") => None,
            // ("connect", "hierarchy-group") => None,
            // ("connect", "hours-of-operation") => None,
            // ("connect", "instance") => None,
            // ("connect", "phone-number") => None,
            // ("connect", "queue") => None,
            // ("connect", "quick-connect") => None,
            // ("connect", "routing-profile") => None,
            // ("connect", "security-profile") => None,
            // ("connect", "user") => None,

            // AWS Cost and Usage Report
            // ("cur", "cur") => None,

            // AWS Glue DataBrew
            // ("databrew", "Dataset") => None,
            // ("databrew", "Job") => None,
            // ("databrew", "Project") => None,
            // ("databrew", "Recipe") => None,
            // ("databrew", "Schedule") => None,

            // AWS Data Exchange
            // ("dataexchange", "assets") => None,
            // ("dataexchange", "data-sets") => None,
            // ("dataexchange", "jobs") => None,
            // ("dataexchange", "revisions") => None,

            // DataSync
            // ("datasync", "agent") => None,
            // ("datasync", "location") => None,
            // ("datasync", "task") => None,
            // ("datasync", "taskexecution") => None,

            // Amazon DynamoDB Accelerator (DAX)
            // ("dax", "application") => None,

            // AWS DeepComposer
            // ("deepcomposer", "audio") => None,
            // ("deepcomposer", "composition") => None,
            // ("deepcomposer", "model") => None,

            // AWS DeepLens
            // ("deeplens", "device") => None,
            // ("deeplens", "model") => None,
            // ("deeplens", "project") => None,

            // AWS DeepRacer
            // ("deepracer", "evaluation_job") => None,
            // ("deepracer", "leaderboard") => None,
            // ("deepracer", "leaderboard_evaluation_job") => None,
            // ("deepracer", "reinforcement_learning_model") => None,
            // ("deepracer", "track") => None,
            // ("deepracer", "training_job") => None,

            // Amazon Detective
            // ("detective", "Graph") => None,

            // AWS Device Farm
            // ("devicefarm", "artifact") => None,
            // ("devicefarm", "device") => None,
            // ("devicefarm", "deviceinstance") => None,
            // ("devicefarm", "devicepool") => None,
            // ("devicefarm", "instanceprofile") => None,
            // ("devicefarm", "job") => None,
            // ("devicefarm", "networkprofile") => None,
            // ("devicefarm", "project") => None,
            // ("devicefarm", "run") => None,
            // ("devicefarm", "sample") => None,
            // ("devicefarm", "session") => None,
            // ("devicefarm", "suite") => None,
            // ("devicefarm", "test") => None,
            // ("devicefarm", "testgrid-project") => None,
            // ("devicefarm", "testgrid-session") => None,
            // ("devicefarm", "upload") => None,
            // ("devicefarm", "vpceconfiguration") => None,

            // AWS Direct Connect
            // ("directconnect", "dx-gateway") => None,
            // ("directconnect", "dxcon") => None,
            // ("directconnect", "dxlag") => None,
            // ("directconnect", "dxvif") => None,

            // Amazon Data Lifecycle Manager
            // ("dlm", "policy") => None,

            // AWS Database Migration Service
            // ("dms", "Certificate") => None,
            // ("dms", "Endpoint") => None,
            // ("dms", "EventSubscription") => None,
            // ("dms", "ReplicationInstance") => None,
            // ("dms", "ReplicationSubnetGroup") => None,
            // ("dms", "ReplicationTask") => None,
            // ("dms", "ReplicationTaskAssessmentRun") => None,
            // ("dms", "ReplicationTaskIndividualAssessment") => None,

            // AWS Directory Service
            // ("ds", "directory") => None,

            // Amazon DynamoDB
            // ("dynamodb", "backup") => None,
            // ("dynamodb", "export") => None,
            // ("dynamodb", "global-table") => None,
            // ("dynamodb", "index") => None,
            // ("dynamodb", "stream") => None,
            ("dynamodb", "table") => Some(format!(
                "https://{region}.{domain}/dynamodbv2/home?region={region}#table?name={resource}",
                region = self.region(),
                domain = self.domain()?,
                resource = self.resource_id(),
            )),

            // AWS Elastic Compute Cloud
            // ("ec2", "capacity-reservation") => None,
            // ("ec2", "carrier-gateway") => None,
            // ("ec2", "client-vpn-endpoint") => None,
            // ("ec2", "customer-gateway") => None,
            // ("ec2", "dedicated-host") => None,
            // ("ec2", "dhcp-options") => None,
            // ("ec2", "egress-only-internet-gateway") => None,
            // ("ec2", "elastic-gpu") => None,
            // ("ec2", "elastic-ip") => None,
            // ("ec2", "export-image-task") => None,
            // ("ec2", "export-instance-task") => None,
            // ("ec2", "fleet") => None,
            // ("ec2", "fpga-image") => None,
            // ("ec2", "host-reservation") => None,
            ("ec2", "image") => Some(format!(
                "https://{region}.{domain}/ec2/home?region={region}#ImageDetails:imageId={resource}",
                region = self.region(),
                domain = self.domain()?,
                resource = self.resource_id(),
            )),
            // ("ec2", "import-image-task") => None,
            // ("ec2", "import-snapshot-task") => None,
            ("ec2", "instance") => Some(format!(
                "https://{region}.{domain}/ec2/home?region={region}#InstanceDetails:instanceId={resource}",
                region = self.region(),
                domain = self.domain()?,
                resource = self.resource_id(),
            )),
            // ("ec2", "internet-gateway") => None,
            // ("ec2", "ipv4pool-ec2") => None,
            // ("ec2", "ipv6pool-ec2") => None,
            // ("ec2", "key-pair") => None,
            ("ec2", "launch-template") => Some(format!(
                "https://{region}.{domain}/ec2/home?region={region}#LaunchTemplateDetails:launchTemplateId={resource}",
                region = self.region(),
                domain = self.domain()?,
                resource = self.resource_id(),
            )),
            // ("ec2", "local-gateway") => None,
            // ("ec2", "local-gateway-route-table") => None,
            // ("ec2", "local-gateway-route-table-virtual-interface-group-association") => None,
            // ("ec2", "local-gateway-route-table-vpc-association") => None,
            // ("ec2", "local-gateway-virtual-interface") => None,
            // ("ec2", "local-gateway-virtual-interface-group") => None,
            ("ec2", "natgateway") => Some(format!(
                "https://{region}.{domain}/vpcconsole/home?region={region}#NatGatewayDetails:natGatewayId={resource}",
                domain = self.domain()?,
                region = self.region(),
                resource = self.resource_id(),
            )),
            // ("ec2", "network-acl") => None,
            // ("ec2", "network-insights-analysis") => None,
            // ("ec2", "network-insights-path") => None,
            // ("ec2", "network-interface") => None,
            // ("ec2", "placement-group") => None,
            // ("ec2", "prefix-list") => None,
            // ("ec2", "reserved-instances") => None,
            // ("ec2", "route-table") => None,
            ("ec2", "security-group") => Some(format!(
                "https://{region}.{domain}/vpc/home?region={region}#SecurityGroup:groupId={resource}",
                domain = self.domain()?,
                region = self.region(),
                resource = self.resource_id(),
            )),
            ("ec2", "snapshot") => Some(format!(
                "https://{region}.{domain}/ec2/home?region={region}#SnapshotDetails:snapshotId={resource}",
                region = self.region(),
                domain = self.domain()?,
                resource = self.resource_id(),
            )),
            // ("ec2", "spot-fleet-request") => None,
            // ("ec2", "spot-instances-request") => None,
            ("ec2", "subnet") => Some(format!(
                "https://{region}.{domain}/vpc/home?region={region}#SubnetDetails:subnetId={resource}",
                domain = self.domain()?,
                region = self.region(),
                resource = self.resource_id(),
            )),
            // ("ec2", "traffic-mirror-filter") => None,
            // ("ec2", "traffic-mirror-filter-rule") => None,
            // ("ec2", "traffic-mirror-session") => None,
            // ("ec2", "traffic-mirror-target") => None,
            // ("ec2", "transit-gateway") => None,
            // ("ec2", "transit-gateway-attachment") => None,
            // ("ec2", "transit-gateway-connect-peer") => None,
            // ("ec2", "transit-gateway-multicast-domain") => None,
            // ("ec2", "transit-gateway-route-table") => None,
            ("ec2", "volume") => Some(format!(
                "https://{region}.{domain}/ec2/home?region={region}#VolumeDetails:volumeId={resource}",
                region = self.region(),
                domain = self.domain()?,
                resource = self.resource_id(),
            )),
            ("ec2", "vpc") => Some(format!(
                "https://{region}.{domain}/vpc/home?region={region}#VpcDetails:VpcId={resource}",
                domain = self.domain()?,
                region = self.region(),
                resource = self.resource_id(),
            )),
            ("ec2", "vpc-endpoint") => Some(format!(
                "https://{region}.{domain}/vpcconsole/home?region={region}#EndpointDetails:vpcEndpointId={resource}",
                domain = self.domain()?,
                region = self.region(),
                resource = self.resource_id(),
            )),
            // ("ec2", "vpc-endpoint-service") => None,
            // ("ec2", "vpc-flow-log") => None,
            // ("ec2", "vpc-peering-connection") => None,
            // ("ec2", "vpn-connection") => None,
            // ("ec2", "vpn-gateway") => None,

            // Amazon Elastic Container Registry
            // ("ecr", "repository") => None,

            // Amazon Elastic Container Registry Public
            // ("ecr-public", "registry") => None,
            // ("ecr-public", "repository") => None,

            // Amazon Elastic Container Service
            // ("ecs", "capacity-provider") => None,
            ("ecs", "cluster") => Some(format!(
                "https://{region}.{domain}/ecs/v2/clusters/{resource}?region={region}",
                region = self.region(),
                domain = self.domain()?,
                resource = self.resource_id(),
            )),
            // ("ecs", "container-instance") => None,
            ("ecs", "service") => {
                let (path_all_but_last, path_last) =
                    self.resource_id().rsplit_once('/').unwrap_or_default();
                Some(format!(
                    "https://{region}.{domain}/ecs/v2/clusters/{path_all_but_last}/services/{path_last}?region={region}",
                    region = self.region(),
                    domain = self.domain()?,
                    path_all_but_last = path_all_but_last,
                    path_last = path_last,
                ))
            }
            ("ecs", "task") => {
                let (path_all_but_last, path_last) =
                    self.resource_id().rsplit_once('/').unwrap_or_default();
                Some(format!(
                    "https://{region}.{domain}/ecs/v2/clusters/{path_all_but_last}/tasks/{path_last}?region={region}",
                    region = self.region(),
                    domain = self.domain()?,
                    path_all_but_last = path_all_but_last,
                    path_last = path_last,
                ))
            }
            ("ecs", "task-definition") => Some(format!(
                "https://{region}.{domain}/ecs/v2/task-definitions/{resource}/{resource_revision}?region={region}",
                region = self.region(),
                domain = self.domain()?,
                resource = self.resource_id(),
                resource_revision = self.resource_revision(),
            )),
            // ("ecs", "task-set") => None,

            // Amazon Elastic Container Service for Kubernetes
            // ("eks", "addon") => None,
            ("eks", "cluster") => Some(format!(
                "https://{domain}/eks/home?region={region}#/clusters/{resource}",
                domain = self.domain()?,
                region = self.region(),
                resource = self.resource_id(),
            )),
            // ("eks", "fargateprofile") => None,
            ("eks", "nodegroup") => {
                let mut parts = self.resource_id().split('/');
                Some(format!(
                    "https://{domain}/eks/home?region={region}#/clusters/{cluster_name}/nodegroups/{nodegroup_name}",
                    domain = self.domain()?,
                    region = self.region(),
                    cluster_name = parts.next()?,
                    nodegroup_name = parts.next()?,
                ))
            }

            // Amazon Elastic Inference
            // ("elastic-inference", "accelerator") => None,
            // ("elastic-inference", "elastic-inference") => None,

            // Amazon ElastiCache
            // ("elasticache", "cluster") => None,
            // ("elasticache", "globalreplicationgroup") => None,
            // ("elasticache", "parametergroup") => None,
            // ("elasticache", "replicationgroup") => None,
            // ("elasticache", "reserved-instance") => None,
            // ("elasticache", "securitygroup") => None,
            // ("elasticache", "snapshot") => None,
            // ("elasticache", "subnetgroup") => None,
            // ("elasticache", "user") => None,
            // ("elasticache", "usergroup") => None,

            // AWS Elastic Beanstalk
            // ("elasticbeanstalk", "application") => None,
            // ("elasticbeanstalk", "applicationversion") => None,
            // ("elasticbeanstalk", "configurationtemplate") => None,
            // ("elasticbeanstalk", "environment") => None,
            // ("elasticbeanstalk", "platform") => None,
            // ("elasticbeanstalk", "solutionstack") => None,

            // Amazon Elastic File System
            // ("elasticfilesystem", "access-point") => None,
            // ("elasticfilesystem", "file-system") => None,

            // Elastic Load Balancing
            // ("elasticloadbalancing", "listener-rule/app") => None,
            // ("elasticloadbalancing", "listener-rule/net") => None,
            // ("elasticloadbalancing", "listener/app") => None,
            // ("elasticloadbalancing", "listener/net") => None,
            ("elasticloadbalancing", "loadbalancer") => Some(format!(
                "https://{region}.{domain}/ec2/home?region={region}#LoadBalancer:loadBalancerArn={arn}",
                region = self.region(),
                domain = self.domain()?,
                arn = self.build(),
            )),
            // ("elasticloadbalancing", "loadbalancer/app/") => None,
            // ("elasticloadbalancing", "loadbalancer/net/") => None,
            // ("elasticloadbalancing", "targetgroup") => None,

            // Amazon Elastic MapReduce
            // ("elasticmapreduce", "cluster") => None,
            // ("elasticmapreduce", "editor") => None,

            // Amazon Elastic Transcoder
            // ("elastictranscoder", "job") => None,
            // ("elastictranscoder", "pipeline") => None,
            // ("elastictranscoder", "preset") => None,

            // AWS Elemental Appliances and Software Activation Service
            // ("elemental-activations", "activation") => None,

            // AWS Elemental Appliances and Software
            // ("elemental-appliances-software", "quote") => None,

            // Amazon EMR on EKS (EMR Containers)
            // ("emr-containers", "jobRun") => None,
            // ("emr-containers", "virtualCluster") => None,

            // Amazon Elasticsearch Service
            // ("es", "domain") => None,

            // Amazon EventBridge
            // ("events", "archive") => None,
            // ("events", "event-bus") => None,
            // ("events", "event-source") => None,
            // ("events", "replay") => None,
            // ("events", "rule") => None,

            // Amazon API Gateway
            // ("execute-api", "execute-api-general") => None,

            // Amazon Kinesis Firehose
            ("firehose", "deliverystream") => Some(format!(
                "https://{domain}/firehose/home?region={region}#/details/{resource}/monitoring",
                domain = self.domain()?,
                region = self.region(),
                resource = self.resource_id(),
            )),

            // AWS Firewall Manager
            // ("fms", "applications-list") => None,
            // ("fms", "policy") => None,
            // ("fms", "protocols-list") => None,

            // Amazon Forecast
            // ("forecast", "algorithm") => None,
            // ("forecast", "dataset") => None,
            // ("forecast", "datasetGroup") => None,
            // ("forecast", "datasetImportJob") => None,
            // ("forecast", "forecast") => None,
            // ("forecast", "forecastExport") => None,
            // ("forecast", "predictor") => None,
            // ("forecast", "predictorBacktestExportJob") => None,

            // Amazon Fraud Detector
            // ("frauddetector", "detector") => None,
            // ("frauddetector", "detector-version") => None,
            // ("frauddetector", "entity-type") => None,
            // ("frauddetector", "event-type") => None,
            // ("frauddetector", "external-model") => None,
            // ("frauddetector", "label") => None,
            // ("frauddetector", "model") => None,
            // ("frauddetector", "model-version") => None,
            // ("frauddetector", "outcome") => None,
            // ("frauddetector", "rule") => None,
            // ("frauddetector", "variable") => None,

            // Amazon FreeRTOS
            // ("freertos", "configuration") => None,

            // Amazon FSx
            // ("fsx", "backup") => None,
            // ("fsx", "file-system") => None,
            // ("fsx", "task") => None,

            // Amazon GameLift
            // ("gamelift", "alias") => None,
            // ("gamelift", "build") => None,
            // ("gamelift", "fleet") => None,
            // ("gamelift", "gameServerGroup") => None,
            // ("gamelift", "gameSessionQueue") => None,
            // ("gamelift", "matchmakingConfiguration") => None,
            // ("gamelift", "matchmakingRuleSet") => None,
            // ("gamelift", "script") => None,

            // Amazon Location
            // ("geo", "geofence-collection") => None,
            // ("geo", "map") => None,
            // ("geo", "place-index") => None,
            // ("geo", "tracker") => None,

            // Amazon Glacier
            // ("glacier", "vault") => None,

            // AWS Global Accelerator
            // ("globalaccelerator", "accelerator") => None,
            // ("globalaccelerator", "endpointgroup") => None,
            // ("globalaccelerator", "listener") => None,

            // AWS Glue
            // ("glue", "catalog") => None,
            // ("glue", "connection") => None,
            // ("glue", "crawler") => None,
            // ("glue", "database") => None,
            // ("glue", "devendpoint") => None,
            // ("glue", "job") => None,
            // ("glue", "mlTransform") => None,
            // ("glue", "registry") => None,
            // ("glue", "schema") => None,
            // ("glue", "table") => None,
            // ("glue", "tableversion") => None,
            // ("glue", "trigger") => None,
            // ("glue", "userdefinedfunction") => None,
            // ("glue", "workflow") => None,

            // Amazon Managed Service for Grafana
            // ("grafana", "workspace") => None,

            // AWS IoT Greengrass V2
            // ("greengrass", "bulkDeployment") => None,
            // ("greengrass", "certificateAuthority") => None,
            // ("greengrass", "component") => None,
            // ("greengrass", "componentVersion") => None,
            // ("greengrass", "connectivityInfo") => None,
            // ("greengrass", "connectorDefinition") => None,
            // ("greengrass", "connectorDefinitionVersion") => None,
            // ("greengrass", "coreDefinition") => None,
            // ("greengrass", "coreDefinitionVersion") => None,
            // ("greengrass", "coreDevice") => None,
            // ("greengrass", "deployment") => None,
            // ("greengrass", "deviceDefinition") => None,
            // ("greengrass", "deviceDefinitionVersion") => None,
            // ("greengrass", "functionDefinition") => None,
            // ("greengrass", "functionDefinitionVersion") => None,
            // ("greengrass", "group") => None,
            // ("greengrass", "groupVersion") => None,
            // ("greengrass", "loggerDefinition") => None,
            // ("greengrass", "loggerDefinitionVersion") => None,
            // ("greengrass", "resourceDefinition") => None,
            // ("greengrass", "resourceDefinitionVersion") => None,
            // ("greengrass", "subscriptionDefinition") => None,
            // ("greengrass", "subscriptionDefinitionVersion") => None,
            // ("greengrass", "thingRuntimeConfig") => None,

            // AWS Ground Station
            // ("groundstation", "Config") => None,
            // ("groundstation", "Contact") => None,
            // ("groundstation", "DataflowEndpointGroup") => None,
            // ("groundstation", "GroundStationResource") => None,
            // ("groundstation", "MissionProfile") => None,
            // ("groundstation", "Satellite") => None,

            // Amazon GuardDuty
            // ("guardduty", "detector") => None,
            // ("guardduty", "filter") => None,
            // ("guardduty", "ipset") => None,
            // ("guardduty", "publishingDestination") => None,
            // ("guardduty", "threatintelset") => None,

            // AWS Health APIs and Notifications
            // ("health", "event") => None,

            // Amazon Honeycode
            // ("honeycode", "screen") => None,
            // ("honeycode", "screen-automation") => None,
            // ("honeycode", "table") => None,
            // ("honeycode", "workbook") => None,

            // AWS Identity and Access Management
            // ("iam", "access-report") => None,
            // ("iam", "assumed-role") => None,
            // ("iam", "federated-user") => None,
            ("iam", "group") => Some(format!(
                "https://{domain}/iamv2/home#/groups/details/{last}",
                domain = self.domain()?,
                last = self.path_last(),
            )),
            // ("iam", "instance-profile") => None,
            // ("iam", "mfa") => None,
            ("iam", "oidc-provider") => Some(format!(
                "https://{domain}/iam/home?#/providers/{string}",
                domain = self.domain()?,
                string = self.build(),
            )),
            ("iam", "policy") => Some(format!(
                "https://{domain}/iam/home?#/policies/{string}",
                domain = self.domain()?,
                string = self.build(),
            )),
            ("iam", "role") => Some(format!(
                "https://{domain}/iam/home?#/roles/{last}",
                domain = self.domain()?,
                last = self.path_last(),
            )),
            // ("iam", "server-certificate") => None,
            // ("iam", "sms-mfa") => None,
            ("iam", "user") => Some(format!(
                "https://{domain}/iam/home?#/users/{resource}",
                domain = self.domain()?,
                resource = self.resource_id(),
            )),

            // Amazon EC2 Image Builder
            // ("imagebuilder", "component") => None,
            // ("imagebuilder", "componentVersion") => None,
            // ("imagebuilder", "containerRecipe") => None,
            // ("imagebuilder", "distributionConfiguration") => None,
            // ("imagebuilder", "image") => None,
            // ("imagebuilder", "imagePipeline") => None,
            // ("imagebuilder", "imageRecipe") => None,
            // ("imagebuilder", "imageVersion") => None,
            // ("imagebuilder", "infrastructureConfiguration") => None,

            // AWS IoT Greengrass
            // ("iot", "authorizer") => None,
            // ("iot", "billinggroup") => None,
            // ("iot", "cacert") => None,
            // ("iot", "cert") => None,
            // ("iot", "client") => None,
            // ("iot", "dimension") => None,
            // ("iot", "domainconfiguration") => None,
            // ("iot", "dynamicthinggroup") => None,
            // ("iot", "fleetmetric") => None,
            // ("iot", "index") => None,
            // ("iot", "job") => None,
            // ("iot", "mitigationaction") => None,
            // ("iot", "otaupdate") => None,
            // ("iot", "policy") => None,
            // ("iot", "provisioningtemplate") => None,
            // ("iot", "rolealias") => None,
            // ("iot", "rule") => None,
            // ("iot", "scheduledaudit") => None,
            // ("iot", "securityprofile") => None,
            // ("iot", "stream") => None,
            // ("iot", "thing") => None,
            // ("iot", "thinggroup") => None,
            // ("iot", "thingtype") => None,
            // ("iot", "topic") => None,
            // ("iot", "topicfilter") => None,
            // ("iot", "tunnel") => None,

            // AWS IoT 1-Click
            // ("iot1click", "device") => None,
            // ("iot1click", "project") => None,

            // AWS IoT Analytics
            // ("iotanalytics", "channel") => None,
            // ("iotanalytics", "dataset") => None,
            // ("iotanalytics", "datastore") => None,
            // ("iotanalytics", "pipeline") => None,

            // AWS IoT Core Device Advisor
            // ("iotdeviceadvisor", "suitedefinition") => None,
            // ("iotdeviceadvisor", "suiterun") => None,

            // AWS IoT Events
            // ("iotevents", "alarmModel") => None,
            // ("iotevents", "detectorModel") => None,
            // ("iotevents", "input") => None,

            // Fleet Hub for AWS IoT Device Management
            // ("iotfleethub", "application") => None,
            // ("iotfleethub", "dashboard") => None,

            // AWS IoT SiteWise
            // ("iotsitewise", "access-policy") => None,
            // ("iotsitewise", "asset") => None,
            // ("iotsitewise", "asset-model") => None,
            // ("iotsitewise", "dashboard") => None,
            // ("iotsitewise", "gateway") => None,
            // ("iotsitewise", "portal") => None,
            // ("iotsitewise", "project") => None,

            // AWS IoT Things Graph
            // ("iotthingsgraph", "System") => None,
            // ("iotthingsgraph", "SystemInstance") => None,
            // ("iotthingsgraph", "Workflow") => None,

            // AWS IoT Core for LoRaWAN
            // ("iotwireless", "Destination") => None,
            // ("iotwireless", "DeviceProfile") => None,
            // ("iotwireless", "ServiceProfile") => None,
            // ("iotwireless", "WirelessDevice") => None,
            // ("iotwireless", "WirelessGateway") => None,

            // Amazon Interactive Video Service
            // ("ivs", "Channel") => None,
            // ("ivs", "Playback-Key-Pair") => None,
            // ("ivs", "Stream-Key") => None,

            // Amazon Managed Streaming for Apache Kafka
            // ("kafka", "cluster") => None,

            // Amazon Kendra
            // ("kendra", "data-source") => None,
            // ("kendra", "faq") => None,
            // ("kendra", "index") => None,
            // ("kendra", "thesaurus") => None,

            // Amazon Kinesis
            // ("kinesis", "consumer") => None,
            // ("kinesis", "stream") => None,

            // Amazon Kinesis Analytics V2
            // ("kinesisanalytics", "application") => None,

            // Amazon Kinesis Video Streams
            // ("kinesisvideo", "channel") => None,
            // ("kinesisvideo", "stream") => None,

            // AWS Key Management Service
            // ("kms", "alias") => None,
            ("kms", "key") => Some(format!(
                "https://{domain}/kms/home?region={region}#/kms/keys/{resource}",
                domain = self.domain()?,
                region = self.region(),
                resource = self.resource_id(),
            )),
            // ("kms", "kmsKey") => None,

            // AWS Lambda
            // ("lambda", "code signing config") => None,
            // ("lambda", "eventSourceMapping") => None,
            ("lambda", "function") => Some(format!(
                "https://{region}.{domain}/lambda/home?region={region}#/functions/{resource}",
                region = self.region(),
                domain = self.domain()?,
                resource = self.resource_id(),
            )),
            // ("lambda", "function alias") => None,
            // ("lambda", "function version") => None,
            ("lambda", "layer") => {
                let (qualifier0, qualifier1) = match self.resource_id().split_once(':') {
                    Some((a, "")) => (a, "1"),
                    Some((a, b)) => (a, b),
                    None => (self.resource_id(), "1"),
                };
                Some(format!(
                    "https://{region}.{domain}/lambda/home?region={region}#/layers/{qualifier0}/versions/{qualifier1}",
                    region = self.region(),
                    domain = self.domain()?,
                    qualifier0 = qualifier0,
                    qualifier1 = qualifier1,
                ))
            }
            // ("lambda", "layerVersion") => None,

            // Amazon Lex V2
            // ("lex", "bot") => None,
            // ("lex", "bot alias") => None,
            // ("lex", "bot version") => None,
            // ("lex", "channel") => None,
            // ("lex", "intent version") => None,
            // ("lex", "slottype version") => None,

            // AWS License Manager
            // ("license-manager", "grant") => None,
            // ("license-manager", "license") => None,
            // ("license-manager", "license-configuration") => None,

            // Amazon Lightsail
            // ("lightsail", "CloudFormationStackRecord") => None,
            // ("lightsail", "Disk") => None,
            // ("lightsail", "DiskSnapshot") => None,
            // ("lightsail", "Domain") => None,
            // ("lightsail", "ExportSnapshotRecord") => None,
            // ("lightsail", "Instance") => None,
            // ("lightsail", "InstanceSnapshot") => None,
            // ("lightsail", "KeyPair") => None,
            // ("lightsail", "LoadBalancer") => None,
            // ("lightsail", "LoadBalancerTlsCertificate") => None,
            // ("lightsail", "PeeredVpc") => None,
            // ("lightsail", "RelationalDatabase") => None,
            // ("lightsail", "RelationalDatabaseSnapshot") => None,
            // ("lightsail", "StaticIp") => None,

            // Amazon CloudWatch Logs
            ("logs", "log-group") => Some(format!(
                "https://{region}.{domain}/cloudwatch/home?region={region}#logsV2:log-groups/log-group/{resource}",
                region = self.region(),
                domain = self.domain()?,
                resource = self
                    .resource_id()
                    .strip_suffix(":*")?
                    .replace(":", "$3A")
                    .replace("#", "$2523")
                    .replace("/", "$252F"),
            )),
            // ("logs", "log-stream") => None,

            // Amazon Lookout for Equipment
            // ("lookoutequipment", "dataset") => None,
            // ("lookoutequipment", "inference-scheduler") => None,
            // ("lookoutequipment", "model") => None,

            // Amazon Lookout for Vision
            // ("lookoutvision", "model") => None,
            // ("lookoutvision", "project") => None,

            // Amazon Machine Learning
            // ("machinelearning", "batchprediction") => None,
            // ("machinelearning", "datasource") => None,
            // ("machinelearning", "evaluation") => None,
            // ("machinelearning", "mlmodel") => None,

            // Amazon Macie
            // ("macie2", "ClassificationJob") => None,
            // ("macie2", "CustomDataIdentifier") => None,
            // ("macie2", "FindingsFilter") => None,
            // ("macie2", "Member") => None,

            // Amazon Managed Blockchain
            // ("managedblockchain", "invitation") => None,
            // ("managedblockchain", "member") => None,
            // ("managedblockchain", "network") => None,
            // ("managedblockchain", "node") => None,
            // ("managedblockchain", "proposal") => None,

            // AWS Elemental MediaConnect
            // ("mediaconnect", "Entitlement") => None,
            // ("mediaconnect", "Flow") => None,
            // ("mediaconnect", "Output") => None,
            // ("mediaconnect", "Source") => None,

            // AWS Elemental MediaConvert
            // ("mediaconvert", "CertificateAssociation") => None,
            // ("mediaconvert", "Job") => None,
            // ("mediaconvert", "JobTemplate") => None,
            // ("mediaconvert", "Preset") => None,
            // ("mediaconvert", "Queue") => None,

            // AWS Elemental MediaLive
            ("medialive", "channel") => Some(format!(
                "https://{region}.{domain}/medialive/home?region={region}#/channels/{resource}",
                region = self.region(),
                domain = self.domain()?,
                resource = self.resource_id(),
            )),
            // ("medialive", "input") => None,
            // ("medialive", "input-device") => None,
            // ("medialive", "input-security-group") => None,
            // ("medialive", "multiplex") => None,
            // ("medialive", "offering") => None,
            // ("medialive", "reservation") => None,

            // AWS Elemental MediaPackage
            // ("mediapackage", "channels") => None,
            // ("mediapackage", "harvest_jobs") => None,
            // ("mediapackage", "origin_endpoints") => None,

            // AWS Elemental MediaPackage VOD
            // ("mediapackage-vod", "assets") => None,
            // ("mediapackage-vod", "packaging-configurations") => None,
            // ("mediapackage-vod", "packaging-groups") => None,

            // AWS Elemental MediaStore
            // ("mediastore", "container") => None,

            // AWS Elemental MediaTailor
            // ("mediatailor", "playbackConfiguration") => None,

            // AWS Migration Hub
            // ("mgh", "migrationTask") => None,
            // ("mgh", "progressUpdateStream") => None,

            // AWS Mobile Hub
            // ("mobilehub", "project") => None,

            // Amazon Pinpoint
            // ("mobiletargeting", "apps") => None,
            // ("mobiletargeting", "campaigns") => None,
            // ("mobiletargeting", "journeys") => None,
            // ("mobiletargeting", "recommenders") => None,
            // ("mobiletargeting", "segments") => None,
            // ("mobiletargeting", "templates") => None,

            // Amazon Monitron
            // ("monitron", "project") => None,

            // Amazon MQ
            // ("mq", "brokers") => None,
            // ("mq", "configurations") => None,

            // Amazon Neptune
            // ("neptune-db", "database") => None,

            // AWS Network Firewall
            // ("network-firewall", "Firewall") => None,
            // ("network-firewall", "FirewallPolicy") => None,
            // ("network-firewall", "StatefulRuleGroup") => None,
            // ("network-firewall", "StatelessRuleGroup") => None,

            // Network Manager
            // ("networkmanager", "connection") => None,
            // ("networkmanager", "device") => None,
            // ("networkmanager", "global-network") => None,
            // ("networkmanager", "link") => None,
            // ("networkmanager", "site") => None,

            // AWS OpsWorks
            // ("opsworks", "stack") => None,

            // AWS Organizations
            // ("organizations", "account") => None,
            // ("organizations", "awspolicy") => None,
            // ("organizations", "handshake") => None,
            // ("organizations", "organization") => None,
            // ("organizations", "organizationalunit") => None,
            // ("organizations", "policy") => None,
            // ("organizations", "root") => None,

            // AWS Panorama
            // ("panorama", "app") => None,
            // ("panorama", "appVersion") => None,
            // ("panorama", "dataSource") => None,
            // ("panorama", "device") => None,
            // ("panorama", "model") => None,

            // Amazon Personalize
            // ("personalize", "algorithm") => None,
            // ("personalize", "batchInferenceJob") => None,
            // ("personalize", "campaign") => None,
            // ("personalize", "dataset") => None,
            // ("personalize", "datasetGroup") => None,
            // ("personalize", "datasetImportJob") => None,
            // ("personalize", "eventTracker") => None,
            // ("personalize", "featureTransformation") => None,
            // ("personalize", "filter") => None,
            // ("personalize", "recipe") => None,
            // ("personalize", "schema") => None,
            // ("personalize", "solution") => None,

            // AWS Performance Insights
            // ("pi", "metric-resource") => None,

            // Amazon Polly
            // ("polly", "lexicon") => None,

            // Amazon Connect Customer Profiles
            // ("profile", "domains") => None,
            // ("profile", "integrations") => None,
            // ("profile", "object-types") => None,

            // AWS Proton
            // ("proton", "environment") => None,
            // ("proton", "environment-template") => None,
            // ("proton", "environment-template-major-version") => None,
            // ("proton", "environment-template-minor-version") => None,
            // ("proton", "service") => None,
            // ("proton", "service-instance") => None,
            // ("proton", "service-template") => None,
            // ("proton", "service-template-major-version") => None,
            // ("proton", "service-template-minor-version") => None,

            // Amazon QLDB
            // ("qldb", "ledger") => None,
            // ("qldb", "stream") => None,

            // Amazon QuickSight
            // ("quicksight", "analysis") => None,
            // ("quicksight", "assignment") => None,
            // ("quicksight", "customization") => None,
            // ("quicksight", "dashboard") => None,
            // ("quicksight", "dataset") => None,
            // ("quicksight", "datasource") => None,
            // ("quicksight", "group") => None,
            // ("quicksight", "ingestion") => None,
            // ("quicksight", "namespace") => None,
            // ("quicksight", "template") => None,
            // ("quicksight", "theme") => None,
            // ("quicksight", "user") => None,

            // AWS Resource Access Manager
            // ("ram", "permission") => None,
            // ("ram", "resource-share") => None,
            // ("ram", "resource-share-invitation") => None,

            // Amazon RDS
            ("rds", "cluster") => Some(format!(
                "https://{domain}/rds/home?region={region}#database:id={resource};is-cluster=true",
                domain = self.domain()?,
                region = self.region(),
                resource = self.resource_id(),
            )),
            // ("rds", "cluster-endpoint") => None,
            // ("rds", "cluster-pg") => None,
            // ("rds", "cluster-snapshot") => None,
            ("rds", "db") => Some(format!(
                "https://{domain}/rds/home?region={region}#database:id={resource}",
                domain = self.domain()?,
                region = self.region(),
                resource = self.resource_id(),
            )),
            // ("rds", "es") => None,
            // ("rds", "global-cluster") => None,
            ("rds", "og") => Some(format!(
                "https://{domain}/rds/home?region={region}#option-group-details:option-group-name={resource}",
                domain = self.domain()?,
                region = self.region(),
                resource = self.resource_id(),
            )),
            // ("rds", "pg") => None,
            // ("rds", "proxy") => None,
            // ("rds", "ri") => None,
            // ("rds", "secgrp") => None,
            ("rds", "snapshot") => Some(format!(
                "https://{domain}/rds/home?region={region}#db-snapshot:id={resource}",
                domain = self.domain()?,
                region = self.region(),
                resource = self.resource_id(),
            )),
            ("rds", "subgrp") => Some(format!(
                "https://{domain}/rds/home?region={region}#db-subnet-group:id={resource}",
                domain = self.domain()?,
                region = self.region(),
                resource = self.resource_id(),
            )),
            // ("rds", "target") => None,
            // ("rds", "target-group") => None,

            // Amazon RDS IAM Authentication
            // ("rds-db", "db-user") => None,

            // Amazon Redshift
            // ("redshift", "cluster") => None,
            // ("redshift", "dbgroup") => None,
            // ("redshift", "dbname") => None,
            // ("redshift", "dbuser") => None,
            // ("redshift", "eventsubscription") => None,
            // ("redshift", "hsmclientcertificate") => None,
            // ("redshift", "hsmconfiguration") => None,
            // ("redshift", "parametergroup") => None,
            // ("redshift", "securitygroup") => None,
            // ("redshift", "securitygroupingress-cidr") => None,
            // ("redshift", "securitygroupingress-ec2securitygroup") => None,
            // ("redshift", "snapshot") => None,
            // ("redshift", "snapshotcopygrant") => None,
            // ("redshift", "snapshotschedule") => None,
            // ("redshift", "subnetgroup") => None,

            // Amazon Rekognition
            // ("rekognition", "collection") => None,
            // ("rekognition", "project") => None,
            // ("rekognition", "projectversion") => None,
            // ("rekognition", "streamprocessor") => None,

            // AWS Resource Groups
            // ("resource-groups", "group") => None,

            // AWS RoboMaker
            // ("robomaker", "deploymentFleet") => None,
            // ("robomaker", "deploymentJob") => None,
            // ("robomaker", "robot") => None,
            // ("robomaker", "robotApplication") => None,
            // ("robomaker", "simulationApplication") => None,
            // ("robomaker", "simulationJob") => None,
            // ("robomaker", "simulationJobBatch") => None,
            // ("robomaker", "world") => None,
            // ("robomaker", "worldExportJob") => None,
            // ("robomaker", "worldGenerationJob") => None,
            // ("robomaker", "worldTemplate") => None,

            // Amazon Route 53
            // ("route53", "change") => None,
            // ("route53", "delegationset") => None,
            ("route53", "healthcheck") => Some(format!(
                "https://{domain}/route53/healthchecks/home",
                domain = self.domain()?,
            )),
            ("route53", "hostedzone") => Some(format!(
                "https://{domain}/route53/home?#resource-record-sets:{resource}",
                domain = self.domain()?,
                resource = self.resource_id(),
            )),
            // ("route53", "queryloggingconfig") => None,
            ("route53", "trafficpolicy") => Some(format!(
                "https://{domain}/route53/trafficflow/home#/policy/{resource}",
                domain = self.domain()?,
                resource = self.resource_id(),
            )),
            ("route53", "trafficpolicyinstance") => Some(format!(
                "https://{domain}/route53/trafficflow/home#/modify-records/edit/{resource}",
                domain = self.domain()?,
                resource = self.resource_id(),
            )),

            // Amazon Route 53 Resolver
            // ("route53resolver", "resolver-dnssec-config") => None,
            // ("route53resolver", "resolver-endpoint") => None,
            // ("route53resolver", "resolver-query-log-config") => None,
            // ("route53resolver", "resolver-rule") => None,

            // Amazon Simple Storage Service (S3)
            ("s3", "") => Some(format!(
                "https://s3.{domain}/s3/buckets/{resource}",
                domain = self.domain()?,
                resource = self.resource_id(),
            )),
            // ("s3", "accesspoint") => None,
            // ("s3", "bucket") => None,
            // ("s3", "job") => None,
            // ("s3", "object") => None,
            // ("s3", "storagelensconfiguration") => None,

            // Amazon S3 on Outposts
            // ("s3-outposts", "accesspoint") => None,
            // ("s3-outposts", "bucket") => None,
            // ("s3-outposts", "endpoint") => None,
            // ("s3-outposts", "object") => None,

            // Amazon SageMaker
            // ("sagemaker", "action") => None,
            // ("sagemaker", "algorithm") => None,
            // ("sagemaker", "app") => None,
            // ("sagemaker", "app-image-config") => None,
            // ("sagemaker", "artifact") => None,
            // ("sagemaker", "automl-job") => None,
            // ("sagemaker", "code-repository") => None,
            // ("sagemaker", "compilation-job") => None,
            // ("sagemaker", "context") => None,
            // ("sagemaker", "data-quality-job-definition") => None,
            // ("sagemaker", "device") => None,
            // ("sagemaker", "device-fleet") => None,
            // ("sagemaker", "domain") => None,
            // ("sagemaker", "edge-packaging-job") => None,
            // ("sagemaker", "endpoint") => None,
            // ("sagemaker", "endpoint-config") => None,
            // ("sagemaker", "experiment") => None,
            // ("sagemaker", "experiment-trial") => None,
            // ("sagemaker", "experiment-trial-component") => None,
            // ("sagemaker", "feature-group") => None,
            // ("sagemaker", "flow-definition") => None,
            // ("sagemaker", "human-loop") => None,
            // ("sagemaker", "human-task-ui") => None,
            // ("sagemaker", "hyper-parameter-tuning-job") => None,
            // ("sagemaker", "image") => None,
            // ("sagemaker", "image-version") => None,
            // ("sagemaker", "labeling-job") => None,
            // ("sagemaker", "model") => None,
            // ("sagemaker", "model-bias-job-definition") => None,
            // ("sagemaker", "model-explainability-job-definition") => None,
            // ("sagemaker", "model-package") => None,
            // ("sagemaker", "model-package-group") => None,
            // ("sagemaker", "model-quality-job-definition") => None,
            // ("sagemaker", "monitoring-schedule") => None,
            // ("sagemaker", "notebook-instance") => None,
            // ("sagemaker", "notebook-instance-lifecycle-config") => None,
            // ("sagemaker", "pipeline") => None,
            // ("sagemaker", "pipeline-execution") => None,
            // ("sagemaker", "processing-job") => None,
            // ("sagemaker", "project") => None,
            // ("sagemaker", "training-job") => None,
            // ("sagemaker", "transform-job") => None,
            // ("sagemaker", "user-profile") => None,
            // ("sagemaker", "workforce") => None,
            // ("sagemaker", "workteam") => None,

            // AWS Savings Plans
            // ("savingsplans", "savingsplan") => None,

            // Amazon EventBridge Schemas
            // ("schemas", "discoverer") => None,
            // ("schemas", "registry") => None,
            // ("schemas", "schema") => None,

            // Amazon SimpleDB
            // ("sdb", "domain") => None,

            // AWS Secrets Manager
            ("secretsmanager", "secret") => {
                let (name, _) = self
                    .resource_id()
                    .rsplit_once('-')
                    .filter(|(_, suffix)| suffix.len() == 6)?;
                Some(format!(
                    "https://{region}.{domain}/{service}/secret?name={name}",
                    region = self.region(),
                    domain = self.domain()?,
                    service = self.service(),
                    name = name,
                ))
            }

            // AWS Security Hub
            // ("securityhub", "hub") => None,
            // ("securityhub", "product") => None,

            // AWS Serverless Application Repository
            // ("serverlessrepo", "applications") => None,

            // AWS Service Catalog
            // ("servicecatalog", "Application") => None,
            // ("servicecatalog", "AttributeGroup") => None,

            // AWS Cloud Map
            // ("servicediscovery", "namespace") => None,
            // ("servicediscovery", "service") => None,

            // Service Quotas
            // ("servicequotas", "quota") => None,

            // Amazon SES
            // ("ses", "configuration-set") => None,
            // ("ses", "custom-verification-email-template") => None,
            // ("ses", "dedicated-ip-pool") => None,
            // ("ses", "deliverability-test-report") => None,
            // ("ses", "event-destination") => None,
            // ("ses", "identity") => None,
            // ("ses", "receipt-filter") => None,
            // ("ses", "receipt-rule") => None,
            // ("ses", "receipt-rule-set") => None,
            // ("ses", "template") => None,

            // AWS Shield
            // ("shield", "attack") => None,
            // ("shield", "protection") => None,

            // AWS Signer
            // ("signer", "signing-job") => None,
            // ("signer", "signing-profile") => None,

            // Amazon SNS
            ("sns", "") => Some(format!(
                "https://{domain}/sns/v3/home?region={region}#/topic/{arn}",
                domain = self.domain()?,
                region = self.region(),
                arn = self.build(),
            )),

            // Amazon SQS
            ("sqs", "") => Some(format!(
                "https://{region}.{domain}/sqs/v2/home?region={region}#/queues/https%3A%2F%2Fsqs.{region}.amazonaws.com%2F{account}%2F{resource}",
                region = self.region(),
                domain = self.domain()?,
                account = self.account(),
                resource = self.resource_id(),
            )),

            // AWS Systems Manager
            // ("ssm", "association") => None,
            // ("ssm", "automation-definition") => None,
            // ("ssm", "automation-execution") => None,
            // ("ssm", "document") => None,
            // ("ssm", "maintenancewindow") => None,
            // ("ssm", "managed-instance") => None,
            // ("ssm", "managed-instance-inventory") => None,
            // ("ssm", "opsitem") => None,
            // ("ssm", "opsmetadata") => None,
            // ("ssm", "parameter") => None,
            // ("ssm", "patchbaseline") => None,
            // ("ssm", "resourcedatasync") => None,
            // ("ssm", "servicesetting") => None,
            // ("ssm", "session") => None,
            // ("ssm", "windowtarget") => None,
            // ("ssm", "windowtask") => None,

            // AWS SSO
            // ("sso", "Account") => None,
            // ("sso", "Instance") => None,
            // ("sso", "PermissionSet") => None,

            // AWS Step Functions
            // ("states", "activity") => None,
            ("states", "execution") => Some(format!(
                "https://{region}.{domain}/states/home?region={region}#/v2/executions/details/{string}",
                region = self.region(),
                domain = self.domain()?,
                string = self.build(),
            )),
            ("states", "stateMachine") => Some(format!(
                "https://{region}.{domain}/states/home?region={region}#/statemachines/view/{string}",
                region = self.region(),
                domain = self.domain()?,
                string = self.build(),
            )),

            // Amazon Storage Gateway
            // ("storagegateway", "device") => None,
            // ("storagegateway", "gateway") => None,
            // ("storagegateway", "share") => None,
            // ("storagegateway", "tape") => None,
            // ("storagegateway", "tapepool") => None,
            // ("storagegateway", "target") => None,
            // ("storagegateway", "volume") => None,

            // Amazon Sumerian
            // ("sumerian", "project") => None,

            // Amazon Simple Workflow Service
            // ("swf", "domain") => None,

            // Amazon CloudWatch Synthetics
            // ("synthetics", "canary") => None,

            // Amazon Timestream
            // ("timestream", "database") => None,
            // ("timestream", "table") => None,

            // AWS Transfer for SFTP
            // ("transfer", "server") => None,
            // ("transfer", "user") => None,

            // AWS Trusted Advisor
            // ("trustedadvisor", "checks") => None,

            // AWS WAF
            // ("waf", "bytematchset") => None,
            // ("waf", "geomatchset") => None,
            // ("waf", "ipset") => None,
            // ("waf", "ratebasedrule") => None,
            // ("waf", "regexmatchset") => None,
            // ("waf", "regexpatternset") => None,
            // ("waf", "rule") => None,
            // ("waf", "rulegroup") => None,
            // ("waf", "sizeconstraintset") => None,
            // ("waf", "sqlinjectionmatchset") => None,
            // ("waf", "webacl") => None,
            // ("waf", "xssmatchset") => None,

            // AWS WAF Regional
            // ("waf-regional", "bytematchset") => None,
            // ("waf-regional", "geomatchset") => None,
            // ("waf-regional", "ipset") => None,
            // ("waf-regional", "ratebasedrule") => None,
            // ("waf-regional", "regexmatchset") => None,
            // ("waf-regional", "regexpatternset") => None,
            // ("waf-regional", "rule") => None,
            // ("waf-regional", "rulegroup") => None,
            // ("waf-regional", "sizeconstraintset") => None,
            // ("waf-regional", "sqlinjectionmatchset") => None,
            // ("waf-regional", "webacl") => None,
            // ("waf-regional", "xssmatchset") => None,

            // AWS WAF V2
            ("wafv2", "global") => Some(format!(
                "https://{domain}/wafv2/homev2/web-acl/{resource}/overview?region=global",
                domain = self.domain()?,
                resource = self.resource_id().replace("webacl/", ""),
            )),
            // ("wafv2", "ipset") => None,
            // ("wafv2", "regexpatternset") => None,
            ("wafv2", "regional") => Some(format!(
                "https://{domain}/wafv2/homev2/web-acl/{resource}/overview?region={region}",
                domain = self.domain()?,
                resource = self.resource_id().replace("webacl/", ""),
                region = self.region(),
            )),
            // ("wafv2", "rulegroup") => None,
            // ("wafv2", "webacl") => None,

            // AWS Well-Architected Tool
            // ("wellarchitected", "workload") => None,

            // Amazon WorkLink
            // ("worklink", "fleet") => None,

            // Amazon WorkMail
            // ("workmail", "organization") => None,

            // Amazon WorkMail Message Flow
            // ("workmailmessageflow", "RawMessage") => None,

            // Amazon WorkSpaces
            // ("workspaces", "directoryid") => None,
            // ("workspaces", "workspacebundle") => None,
            // ("workspaces", "workspaceid") => None,
            // ("workspaces", "workspaceipgroup") => None,

            // AWS X-Ray
            // ("xray", "group") => None,
            // ("xray", "sampling-rule") => None,

            // Nothing matched
            _ => None,
        }
    }
}

/// Provides private helper methods for the provided methods of the `ArnParts` trait.
trait ArnPartsHelper<'a> {
    fn domain(&self) -> Option<&str>;
    fn path_last(&self) -> &str;
}

impl<'a, T: ArnParts<'a>> ArnPartsHelper<'a> for T {
    /// Returns the base console domain for the partition.
    ///
    /// Returns None if we don't know the domain for this partition.
    /// Partitions are pretty well-known, so None means that the partition
    /// is most likely invalid.
    fn domain(&self) -> Option<&str> {
        // https://github.com/boto/botocore/blob/master/botocore/data/endpoints.json
        match self.partition() {
            "aws" => Some("console.aws.amazon.com"),

            // Untested
            "aws-cn" => Some("console.amazonaws.cn"),
            "aws-us-gov" => Some("console.amazonaws-us-gov.com"),

            // Unknown partition
            _ => None,
        }
    }

    /// If the resource part represents a path, then returns the last
    /// component of it, else returns the entire resource part.
    ///
    /// Example (assuming `.has_path()` is `true`):
    ///
    /// ```text
    /// Input: "aws-service-role/support.amazonaws.com/AWSServiceRoleForSupport"
    /// Output: "AWSServiceRoleForSupport"
    /// ```
    fn path_last(&self) -> &str {
        if self.has_path() {
            self.resource_id()
                .rsplit('/')
                .next()
                .unwrap_or(self.resource_id())
        } else {
            self.resource_id()
        }
    }
}
