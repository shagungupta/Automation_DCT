import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.delay(5)

WebUI.maximizeWindow()

WebUI.navigateToUrl('https://answersqc.nielsen.com/gateway/logon.htm?TYPE=33619969&REALMOID=06-0005834e-6f29-1411-9454-05e30a2730e6&GUID=&SMAUTHREASON=0&METHOD=GET&SMAGENTNAME=-SM-9sDC6fas65vzIfJP1vnAq6EuJKD8Kgzhco%2b64KDWCdW8BOgcJ7BsA3QuEfs4cfzt&TARGET=-SM-https%3a%2f%2foperationsuat3%2enielsen%2ecom%2fCDARHome%2f#/login/')

WebUI.click(findTestObject('Login/Page_Nielsen Answers Login/div_By using this site you consent to the use of cookies For more information please see our                Cookie Policy'))

WebUI.setText(findTestObject('Login/Page_Nielsen Answers Login/input_Nielsen Answers Login_USER'), 'shagun.gupta.consultant@nielsen.com')

WebUI.setEncryptedText(findTestObject('Login/Page_Nielsen Answers Login/input_Nielsen Answers Login_PASSWORD'), 
    'y0uAm98JxwZRShil0uEImg==')

WebUI.sendKeys(findTestObject('Login/Page_Nielsen Answers Login/input_Nielsen Answers Login_PASSWORD'), 
    Keys.chord(Keys.ENTER))

WebUI.click(findTestObject('Login/Page_Nielsen Operations/img_Nielsen Command Center_gridpanelImageLanding'))

WebUI.switchToWindowTitle('Digital Collection Tool')

WebUI.click(findTestObject('Login/Page_Digital Collection Tool/img_Client Customization_image-1044'))

WebUI.click(findTestObject('Login/Page_Digital Collection Tool/span_OK'))

WebUI.delay(10)

WebUI.closeBrowser()

