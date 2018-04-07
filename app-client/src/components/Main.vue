<template>
  <el-container>
    <el-header>
      <h1>Rocket Todo App</h1>
    </el-header>
    <el-main>
      <el-form :inline="true" class="task-form-inline">
        <el-form-item label="Task name">
          <el-input size="30" v-model="name"></el-input>
        </el-form-item>
        <el-form-item label="Due">
          <el-input placeholder="YYYY-MM-DD" v-model="due"></el-input>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="submitTask()">Submit</el-button>
        </el-form-item>
      </el-form>

      <!--task list-->
      <ul>
        <li v-for="task in tasks">
          <el-checkbox v-model="task.done" v-on:change="changeStatus(task)">{{ task.name }} : {{ task.due }}</el-checkbox>
        </li>
      </ul>

      <!--dialog-->
      <el-dialog
        title="Error"
        :visible.sync="errorDialogVisible"
        width="30%">
        <span>An error has been occurred. Please check console log.</span>
        <span slot="footer" class="dialog-footer">
          <el-button @click="errorDialogVisible = false">OK</el-button>
        </span>
      </el-dialog>
    </el-main>
  </el-container>
</template>

<script>
import axios from 'axios'

export default {
  data() {
    return {
      tasks: [],
      name: '',
      due: '',
      errorDialogVisible: false,
    }
  },
  methods: {
    submitTask: function() {
      const self = this
      let task = {
        name: self.name,
        due: self.due,
        done: false
      }

      axios.post(`http://localhost:8000/tasks/new`, task, {
        headers: {
          'Content-Type': 'application/json',
        }
      }).then(response => {
        console.log(response)
        self.tasks.push(response.data)
        self._clearForm()
      }).catch(err => {
        self.errorDialogVisible = true
      })
    },
    _clearForm: function() {
      this.name = ''
      this.due = ''
    },
    changeStatus: function(task) {
      const self = this
      axios.patch(`http://localhost:8000/tasks/update`, task, {
        headers: {
          'Content-Type':'application/json',
        }
      }).catch(err => {
        self.errorDialogVisible = true
      })
    }
  },
  created: function() {
    const self = this
    axios.get(`http://localhost:8000/tasks/list`).then(response => {
      response.data.forEach((v, i, a) => {
        self.tasks.push(v)
      })
    }).catch(err => {
      console.error(err)
      self.errorDialogVisible = true
    })
  }
}
</script>
