<template>
  <div id="app" style="max-width: 600px; margin: auto; padding: 20px;">
    <h2>Рассылка писем</h2>

    <div style="margin-bottom: 20px;">
      <button 
        :class="{ active: currentTab === 'send' }" 
        @click="currentTab = 'send'">Рассылка</button>
      <button 
        :class="{ active: currentTab === 'settings' }" 
        @click="currentTab = 'settings'" 
        style="margin-left: 10px;">Настройки</button>
      <button 
        :class="{ active: currentTab === 'opens' }" 
        @click="currentTab = 'opens'" 
        style="margin-left: 10px;">Открытые письма</button>
    </div>

    <!-- Вкладка Рассылка -->
    <div v-if="currentTab === 'send'">
      <form @submit.prevent="sendBulkEmail">
        <div>
          <label>Email отправителя:</label><br />
          <input type="email" v-model="email" required />
        </div>
        <div>
          <label>Тема письма:</label><br />
          <input type="text" v-model="subject" required />
        </div>

        <div style="display: flex; align-items: center; margin: 10px 0;">
          <label style="margin-right: 10px; min-width: 80px;">Тип письма:</label>

          <label style="display: flex; align-items: center; margin-right: 15px; cursor: pointer;">
            <input type="radio" value="text" v-model="mailType" style="margin-right: 5px; position: relative; top: 5px;" />
            Текстовое
          </label>

          <label style="display: flex; align-items: center; cursor: pointer;">
            <input type="radio" value="html" v-model="mailType" style="margin-right: 5px; position: relative; top: 5px;" />
            HTML
          </label>
        </div>


        <div v-if="mailType === 'text'">
          <label>Текст письма:</label><br />
          <textarea v-model="body" rows="6" required></textarea>
        </div>

        <div v-if="mailType === 'html'">
          <label>HTML код письма:</label><br />
          <textarea v-model="htmlBody" rows="8" required></textarea>

        <div style="display: flex; align-items: center; margin: 10px 0;">
          <label style="display: flex; align-items: center; margin: 10px 0; cursor: pointer;">
            Добавить текстовое тело к HTML письму
            <input
              type="checkbox"
              v-model="addTextToHtml"
              style="margin-left: 10px;"
            />
          </label>

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

      <!-- Список email -->
      <div v-if="emailList.length" style="margin-top: 20px;">
        <h4>Найдено {{ emailList.length }} адресов:</h4>
        <ul style="max-height: 200px; overflow-y: auto; border: 1px solid #ccc; padding: 10px;">
          <li v-for="(email, index) in emailList" :key="index">{{ email }}</li>
        </ul>
      </div>
    </div>

    <!-- Вкладка Настройки -->
    <div v-if="currentTab === 'settings'">
      <h3>Настройки SMTP</h3>
      <div>
        <label>SMTP сервер:</label><br />
        <input type="text" v-model="smtpServer" placeholder="smtp.yandex.ru" required />
      </div>
      <div>
        <label>Порт:</label><br />
        <input type="number" v-model.number="port" placeholder="465" required />
      </div>
      <div>
        <label>Пароль:</label><br />
        <input type="password" v-model="password" required />
      </div>
      <div>
        <label>Минимальная задержка (сек): ВРЕМЕННО НЕ РАБОТАЕТ</label><br />
        <input type="number" v-model.number="minDelay" min="0" />
      </div>
      <div>
        <label>Максимальная задержка (сек): ВРЕМЕННО НЕ РАБОТАЕТ</label><br />
        <input type="number" v-model.number="maxDelay" :min="minDelay" />
      </div>

      <button @click="saveAuth" style="margin-top: 10px;">Сохранить настройки</button>
    </div>

    <!-- Вкладка Открытые письма -->
    <div v-if="currentTab === 'opens'" style="margin-top: 20px;">
      <h3>Открытые письма ВРЕМЕННО НЕ РАБОТАЕТ</h3>
      <ul v-if="openedEmails.length" style="border: 1px solid #ccc; padding: 10px; max-height: 300px; overflow-y: auto;">
        <li v-for="(entry, index) in openedEmails" :key="index">
          {{ entry.email }} — {{ entry.opened_at }}
        </li>
      </ul>
      <div v-else>Нет данных об открытиях</div>
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
      currentTab: 'send',
      email: '',
      password: '',
      smtpServer: '',
      port: 465,
      minDelay: 2,
      maxDelay: 5,
      subject: '',
      body: '',
      htmlBody: '',
      addTextToHtml: false,
      mailType: 'text',
      message: '',
      error: '',
      emailList: [],
      openedEmails: [],
      authFile: 'auth.json',
      opensFile: 'opens.json', // здесь будут сохраняться открытия
      delayMinMs: 500,  // минимальная задержка в миллисекундах (по умолчанию)
      delayMaxMs: 1500, // максимальная задержка в миллисекундах (по умолчанию)
    }
  },
  async mounted() {
    await this.loadAuth()
    await this.loadOpens()
  },
  methods: {
    async saveAuth() {
      const authData = {
        email: this.email,
        password: this.password,
        smtpServer: this.smtpServer,
        port: this.port,
        minDelay: this.minDelay,
        maxDelay: this.maxDelay
      }
      try {
        await writeTextFile(this.authFile, JSON.stringify(authData), {
          dir: BaseDirectory.Config
        })
        this.message = 'Настройки сохранены'
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
        this.minDelay = authData.minDelay || 2
        this.maxDelay = authData.maxDelay || 5
      } catch (e) {
        console.info('auth.json не найден, загрузка пропущена')
      }
    },

    async loadOpens() {
      try {
        const content = await readTextFile(this.opensFile, {
          dir: BaseDirectory.Config
        })
        this.openedEmails = JSON.parse(content)
      } catch (e) {
        console.info('opens.json не найден, загрузка пропущена')
      }
    },

    async sendBulkEmail() {
      this.message = ''
      this.error = ''
      if (!this.emailList.length) {
        this.error = 'Список email-адресов пуст.'
        return
      }

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
            min_delay: this.minDelay,
            max_delay: this.maxDelay
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
/* Градиентный фон */
#app {
  background: linear-gradient(135deg, #1f5f5b);
  color: #ddd;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  min-height: 100vh;
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 0 20px #1f5f5b;
}

/* Заголовки */
h2, h3, h4 {
  color: #061a23;
  font-weight: 600;
}

/* Стили для инпутов и textarea */
input, textarea {
  width: 100%;
  padding: 10px;
  margin-top: 6px;
  margin-bottom: 15px;
  border: 1.5px solid #1f5f5b;
  border-radius: 6px;
  background-color: #49b265;
  color: #eee;
  font-size: 1rem;
  transition: border-color 0.3s ease;
}

input:focus, textarea:focus {
  outline: none;
  border-color: #061a23;
  background-color: #49b265;
}

/* Кнопки */
button {
  background-color: #49b265;
  border: none;
  border-radius: 8px;
  color: #eee;
  cursor: pointer;
  font-weight: 600;
  padding: 12px 20px;
  box-shadow: 0 4px 8px #061a23;
  transition: background-color 0.3s ease, box-shadow 0.2s ease, transform 0.1s ease;
  user-select: none;
}

button:hover {
  background-color: #06373a;
  box-shadow: 0 6px 12px #061a23;
}

button:active {
  background-color: #49b265;
  box-shadow: 0 2px 4px #49b265;
  transform: translateY(2px);
}

/* Стили для кнопок переключения вкладок с текущим выделением */
div[style*="margin-bottom: 20px"] > button {
  background-color: #49b265;
  margin-right: 10px;
}

div[style*="margin-bottom: 20px"] > button.active {
  background-color: #06373a;
  box-shadow: 0 4px 12px #061a23;
}

/* Списки с прокруткой */
ul {
  list-style: none;
  padding-left: 0;
}

/* Метки для checkbox */
input[type="checkbox"] {
  transform: scale(1.2);
  margin-right: 6px;
  cursor: pointer;
}

</style>
