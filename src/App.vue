<template>
  <div id="app" style="max-width: 500px; margin: auto; padding: 20px;">
    <h2>Рассылка писем</h2>
    <form @submit.prevent="sendBulkEmail">
      <div>
        <label>Email отправителя:</label><br />
        <input type="email" v-model="email" required />
      </div>
      <div>
        <label>Пароль:</label><br />
        <input type="password" v-model="password" required />
      </div>
      <div>
        <label>SMTP сервер:</label><br />
        <input type="text" v-model="smtpServer" placeholder="smtp.yandex.ru" required />
      </div>
      <div>
        <label>Порт:</label><br />
        <input type="number" v-model.number="port" placeholder="465" required />
      </div>
      <div>
        <label>Тема письма:</label><br />
        <input type="text" v-model="subject" required />
      </div>

      <div style="margin: 10px 0;">
        <label>Тип письма:</label><br />
        <input type="radio" id="text" value="text" v-model="mailType" />
        <label for="text">Текстовое</label>
        <input type="radio" id="html" value="html" v-model="mailType" style="margin-left: 20px;" />
        <label for="html">HTML</label>
      </div>

      <div v-if="mailType === 'text'">
        <label>Текст письма:</label><br />
        <textarea v-model="body" rows="6" required></textarea>
      </div>

      <div v-if="mailType === 'html'">
        <label>HTML код письма:</label><br />
        <textarea v-model="htmlBody" rows="8" required></textarea>

        <div style="margin-top: 10px;">
          <input type="checkbox" id="addText" v-model="addTextToHtml" />
          <label for="addText">Добавить текстовое тело к HTML письму</label>
        </div>

        <div v-if="addTextToHtml" style="margin-top: 10px;">
          <label>Текст для добавления:</label><br />
          <textarea v-model="body" rows="4"></textarea>
        </div>
      </div>

      <button type="submit" style="margin-top: 15px;">Отправить рассылку</button>
    </form>

    <div v-if="message" style="margin-top: 20px; color: green;">
      {{ message }}
    </div>
    <div v-if="error" style="margin-top: 20px; color: red;">
      {{ error }}
    </div>

    <!-- Загрузка базы -->
    <div style="margin-top: 30px;">
      <h3>Загрузка базы email-адресов</h3>
      <input type="file" @change="handleFileUpload" accept=".txt,.csv,.xlsx" />
    </div>

    <!-- Список -->
    <div v-if="emailList.length" style="margin-top: 20px;">
      <h4>Найдено {{ emailList.length }} адресов:</h4>
      <ul style="max-height: 200px; overflow-y: auto; border: 1px solid #ccc; padding: 10px;">
        <li v-for="(email, index) in emailList" :key="index">{{ email }}</li>
      </ul>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/core'
import * as XLSX from 'xlsx'
import { writeTextFile, readTextFile } from '@tauri-apps/plugin-fs'
import { BaseDirectory } from '@tauri-apps/plugin-fs'

export default {
  data() {
    return {
      email: '',
      password: '',
      smtpServer: '',
      port: 465,
      subject: '',
      body: '',
      htmlBody: '',
      addTextToHtml: false,
      mailType: 'text',
      message: '',
      error: '',
      emailList: [],
      authFile: 'auth.json'
    }
  },
  async mounted() {
    await this.loadAuth()
  },
  methods: {
    async saveAuth() {
      const authData = {
        email: this.email,
        password: this.password,
        smtpServer: this.smtpServer,
        port: this.port
      }
      try {
        await writeTextFile(this.authFile, JSON.stringify(authData), {
          dir: BaseDirectory.Config
        })
      } catch (e) {
        console.error('Ошибка сохранения auth.json:', e)
      }
    },

    async loadAuth() {
      try {
        const content = await readTextFile(this.authFile, {
          dir: BaseDirectory.Config
        })
        const authData = JSON.parse(content)
        this.email = authData.email || ''
        this.password = authData.password || ''
        this.smtpServer = authData.smtpServer || ''
        this.port = authData.port || 465
      } catch (e) {
        // Файл может отсутствовать при первом запуске — это не ошибка
        console.info('auth.json не найден, загрузка пропущена')
      }
    },

    async sendBulkEmail() {
      this.message = ''
      this.error = ''
      if (!this.emailList.length) {
        this.error = 'Список email-адресов пуст.'
        return
      }

      // сохраняем перед отправкой
      await this.saveAuth()

      try {
        this.message = 'Отправка писем...'
        const payload = {
          config: {
            email: this.email,
            password: this.password,
            smtp_server: this.smtpServer,
            port: this.port,
            recipients: this.emailList,
            subject: this.subject,
            mail_type: this.mailType,
            body: this.mailType === 'text' ? this.body : '',
            html_body: this.mailType === 'html' ? this.htmlBody : '',
            add_text_to_html: this.mailType === 'html' ? this.addTextToHtml : false,
            text_for_html: this.mailType === 'html' && this.addTextToHtml ? this.body : '',
          }
        }

        const res = await invoke('send_bulk_email', payload)
        this.message = res
      } catch (e) {
        this.error = 'Ошибка: ' + (e?.toString() || 'неизвестная')
      }
    },

    handleFileUpload(event) {
      const file = event.target.files[0]
      if (!file) return

      const reader = new FileReader()

      if (file.name.endsWith('.xlsx') || file.name.endsWith('.xls')) {
        reader.onload = (e) => {
          const data = new Uint8Array(e.target.result)
          const workbook = XLSX.read(data, { type: 'array' })
          const firstSheetName = workbook.SheetNames[0]
          const worksheet = workbook.Sheets[firstSheetName]
          const json = XLSX.utils.sheet_to_json(worksheet, { header: 1 })

          const flatEmails = json.flat().filter(cell =>
            typeof cell === 'string' && /^[\w.-]+@[\w.-]+\.\w+$/.test(cell)
          )
          this.emailList = [...new Set(flatEmails)]
        }
        reader.readAsArrayBuffer(file)
      } else {
        reader.onload = (e) => {
          const text = e.target.result
          const emails = text
            .split(/[,;\s]+/)
            .filter(email => /^[\w.-]+@[\w.-]+\.\w+$/.test(email))
          this.emailList = [...new Set(emails)]
        }
        reader.readAsText(file)
      }
    }
  }
}
</script>


<style>
input, textarea {
  width: 100%;
  padding: 8px;
  margin-top: 5px;
  margin-bottom: 10px;
  box-sizing: border-box;
}
button {
  padding: 10px 15px;
}
</style>
