<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_By using this site you consent to the use of cookies For more information please see our                Cookie Policy</name>
   <tag></tag>
   <elementGuidId>8ab71e40-77f4-4b3b-b6e2-6c4f511af539</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='containt']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>containt</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
        
			 
                By using this site, you consent to the use of cookies. For more information, please see our
                Cookie Policy
				
			
            
                
                    
                
                
                    WELCOME TO NIELSEN ANSWERS
                
            
        
        
            
                    
                    
                        &lt;img class=&quot;banner&quot; src=&quot;answers/images/slider-background.png&quot; alt=&quot; &quot; />
                    
            
            
                
                    
                        








Nielsen Answers Login

	
	
	
	
	
	 
	
    
        
        
            
        
    
    
        
        
            
        
    
    
        
            Remember Me
            
            
        
    
    
        Login
    



    
        Forgot Password / New to Nielsen
    
	
    
        Need Support Information
    
	


	function recordOutboundLink(link) {
        try {
            setTimeout(function () {
                window.open(link.href, &quot;_blank&quot;);
            }, 100);
        } catch (err) {}
    }

    function loginSubmit() {
		if ($(&quot;#rememberMe&quot;).is(&quot;:checked&quot;) === true) {
			var userName = $(&quot;#USER&quot;).val();
            $.cookie('username', userName, {
                expires: 365,
                domain: &quot;.nielsen.com&quot;,
                path: &quot;/&quot;,
                secure: true
            });
        } else {
			$.removeCookie('username',{
                expires: 365,
                domain: &quot;.nielsen.com&quot;,
                path: &quot;/&quot;,
                secure: true
            });
        }
    }

    var validator = $(&quot;#loginForm&quot;).validate({
		onkeyup : false,
        highlight: function (element, errorClass, validClass) {
            $(element)
                .parent(&quot;div&quot;).parent(&quot;div&quot;)
                .addClass(&quot;warning&quot;)
        },
        unhighlight: function (element, errorClass, validClass) {
            $(element)
                .parent(&quot;div&quot;).parent(&quot;div&quot;)
                .removeClass(&quot;warning&quot;);
        },
        errorPlacement: function (error, element) {
            error.addClass(&quot;nielsen-secondary-input-label&quot;);
            error.appendTo(element.parent(&quot;div&quot;).parent(&quot;div&quot;));
        },
        rules: {
            USER: {
                required: true,
                email: true
            },
            PASSWORD: {
                required: true
            },
        },
        messages: {
            USER: {
                required: &quot;Please enter an email address&quot;,
                email: &quot;Please enter a valid email address&quot;,
            },
            PASSWORD: {
                required: &quot;Please enter the password&quot;,
            }
        }
    });
    $(document).ready(function () {
		if(navigator.userLanguage){
			$(&quot;.nielsen-message&quot;).focus();
		}else{
			$(&quot;#USER&quot;).focus();
		}
        if ($.cookie('username')) {
            $(&quot;#rememberMe&quot;).prop('checked', true);
            $(&quot;#USER&quot;).val($.cookie('username'));
			$(&quot;#PASSWORD&quot;).focus();
        }
    });

                    
                     
                
            
        
        
            
                
					
                        Copyright © 2019 The Nielsen Company
                        The Science Behind What's Next™
                        All Rights Reserved
                        Privacy Notice
                        Terms and Conditions
                    
                
            
        
    </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;containt&quot;)</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='containt']</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[2]</value>
   </webElementXpaths>
</WebElementEntity>
