<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Task-POST</name>
   <tag></tag>
   <elementGuidId>89ce8227-820f-42ff-ab07-f6e4d49fdf2e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;data\&quot;: {\n    \&quot;id\&quot;: \&quot;{{mpdxUUID}}\&quot;,\n    \&quot;type\&quot;: \&quot;tasks\&quot;,\n    \&quot;attributes\&quot;: {\n      \&quot;activity_type\&quot;: \&quot;Call\&quot;,\n      \&quot;comments_count\&quot;: 0,\n      \&quot;completed\&quot;: true,\n      \&quot;completed_at\&quot;: \&quot;{{utcnow}}\&quot;,\n      \&quot;created_at\&quot;: \&quot;{{utcnow}}\&quot;,\n      \&quot;location\&quot;: null,\n      \&quot;next_action\&quot;: null,\n      \&quot;notification_time_before\&quot;: null,\n      \&quot;notification_time_unit\&quot;: null,\n      \&quot;notification_type\&quot;: null,\n      \&quot;result\&quot;: \&quot;Appointment\&quot;,\n      \&quot;starred\&quot;: false,\n      \&quot;start_at\&quot;: \&quot;{{currentTime}}\&quot;,\n      \&quot;subject\&quot;: \&quot;auto-test\&quot;,\n      \&quot;subject_hidden\&quot;: false,\n      \&quot;tag_list\&quot;: [\n\n      ],\n      \&quot;updated_at\&quot;: \&quot;{{utcnow}}\&quot;,\n      \&quot;updated_in_db_at\&quot;: \&quot;{{utcnow}}\&quot;\n    },\n    \&quot;relationships\&quot;: {\n      \&quot;comments\&quot;: {\n        \&quot;data\&quot;: [\n\n        ]\n      },\n      \&quot;contacts\&quot;: {\n        \&quot;data\&quot;: [\n\n        ]\n      },\n      \&quot;people\&quot;: {\n        \&quot;data\&quot;: [\n\n        ]\n      },\n      \&quot;email_addresses\&quot;: {\n        \&quot;data\&quot;: [\n\n        ]\n      },\n      \&quot;phone_numbers\&quot;: {\n        \&quot;data\&quot;: [\n\n        ]\n      },\n      \&quot;activity_contacts\&quot;: {\n        \&quot;data\&quot;: [\n\n        ]\n      },\n      \&quot;account_list\&quot;: {\n        \&quot;data\&quot;: {\n          \&quot;id\&quot;: \&quot;08bb09d1-3b62-4690-9596-b625b8af4750\&quot;,\n          \&quot;type\&quot;: \&quot;account_lists\&quot;\n        }\n      }\n    }\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/vnd.api+json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic dGVzdEB0ZXN0LmNvbTpUZXN0MTIzNA==</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl></restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.null</defaultValue>
      <description></description>
      <id>b1874127-dbc3-4a94-a467-dbda2004b460</id>
      <masked>false</masked>
      <name>variable</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f14729dc-2c65-4231-9a0b-cd61227b6a06</id>
      <masked>false</masked>
      <name>variable_0</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
