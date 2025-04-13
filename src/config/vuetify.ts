import { createVuetify } from 'vuetify'
import 'vuetify/styles'
import '@mdi/font/css/materialdesignicons.css'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'

const customTheme = {
    dark: true,
    colors: {
        background: '#242424',
        surface: '#1a1a1a',
        primary: '#646cff',
        secondary: '#535bf2',
        accent: '#82B1FF',
        error: '#FF5252',
        info: '#2196F3',
        success: '#4CAF50',
        warning: '#FFC107',
        onBackground: '#rgba(255, 255, 255, 0.87)',
        onSurface: '#rgba(255, 255, 255, 0.87)',
    },
}

const vuetify = createVuetify({
    components,
    directives,
    icons: {
        defaultSet: 'mdi',
    },
    theme: {
        defaultTheme: 'customTheme',
        themes: {
            customTheme,
        },
    },
})

export default vuetify
