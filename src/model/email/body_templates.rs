pub fn purchase_mail_template() -> &'static str {
    "<html><head><meta charset='UTF-8'>
            <title>Purchase Successful</title>
            <style>
            body {
              font-family: Arial, sans-serif;
              background-color: #f5f5f5;
              margin: 0;
              padding: 0;
            }
            .container {
              max-width: 600px;
              margin: 0 auto;
              background-color: #ffffff;
              padding: 20px;
            }
            h1 {
              color: #333333;
            }
            p {
              color: #555555;
            }
            .highlight {
              color: #008000;
              font-weight: bold;
            }
          </style>
        </head>
        <body>
        <div class='container'>
            <h1>Thank you for your purchase!</h1>
            <p>Your purchase details:</p>
            <p><strong>Purchase Detail:</strong> <span class='highlight'>{{purchase_detail}}</span></p>
            <p>If you have any questions or need further assistance, please don't hesitate to contact our support team.</p>
            <p>Best regards,</p>
            <p>Your Company Name</p>
        </div>
        </body>
        </html>"
}

pub fn payment_rejected_template() -> &'static str {
    "<html>
        <head>
          <meta http-equiv='Content-Type' content='text/html; charset=utf-8'>
          <meta http-equiv='Content-Style-Type' content='text/css'>
          <title></title>
          <meta name='Generator' content='Cocoa HTML Writer'>
          <meta name='CocoaVersion' content='2113.5'>
          <style type='text/css'>
            body {background-color: #f2f2f2}
            p.p2 {margin: 0.0px 0.0px 12.0px 0.0px; font: 12.0px Arial; color: #434343; -webkit-text-stroke: #434343}
            p.p3 {margin: 0.0px 0.0px 12.0px 0.0px; font: 12.0px Arial; color: #fb0007; -webkit-text-stroke: #fb0007}
            span.s1 {font-kerning: none; background-color: #ffffff}
            span.s2 {font-kerning: none; color: #434343; background-color: #ffffff; -webkit-text-stroke: 0px #434343}
          </style>
        </head>
        <body>
        <h1 style='margin: 0.0px 0.0px 16.1px 0.0px; font: 24.0px Arial; color: #fb0007; -webkit-text-stroke: #fb0007'><span class='s1'><b>Payment Rejected</b></span></h1>
        <p class='p2'><span class='s1'>We regret to inform you that your payment has been rejected.</span></p>
        <p class='p2'><span class='s1'>Event details:</span></p>
        <p class='p3'><span class='s2'><b>Payment detail:</b> </span><span class='s1'><b>{{event_detail}}</b></span></p>
        <p class='p2'><span class='s1'>If you have any questions or need further assistance, please contact our support team.</span></p>
        <p class='p2'><span class='s1'>Best regards,</span></p>
        <p class='p2'><span class='s1'>Your Company Name</span></p>
        </body>
    </html>"
}