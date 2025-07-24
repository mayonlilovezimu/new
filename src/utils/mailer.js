import nodemailer from 'nodemailer';

export async function sendTestEmail({ email, password, smtpServer, port }) {
  try {
    let transporter = nodemailer.createTransport({
      host: smtpServer,
      port: port,
      secure: port === 465,
      auth: {
        user: email,
        pass: password,
      },
    });

    let info = await transporter.sendMail({
      from: email,
      to: email,
      subject: 'Тестовое письмо от emailsender',
      html: '<b>Это тестовое письмо</b> — отправлено через nodemailer!',
    });

    return `Письмо отправлено: ${info.messageId}`;
  } catch (error) {
    throw error;
  }
}
