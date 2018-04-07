<template>
  <el-container>
    <el-header>
      <h1>Rocket Todo App</h1>
    </el-header>
    <el-main>
      <el-form :inline="true" class="task-form-inline">
        <el-form-item label="Task Description">
          <el-input size="30" v-model="description"></el-input>
        </el-form-item>
        <el-form-item label="Due">
          <el-input placeholder="YYYY-MM-DD" v-model="due"></el-input>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="submitTask()">Submit</el-button>
        </el-form-item>
      </el-form>
    </el-main>
  </el-container>
</template>

<script>
import axios from 'axios'

export default {
  data() {
    return {
      description: '',
      due: '',
    }
  },
  methods: {
    submitTask: function() {
      const self = this
      let task = {
        name: self.description,
        due: self.due,
        done: false
      }

      axios.post(`http://localhost:8000/tasks/new`, task, {
        headers: {
          'Content-Type': 'application/json',
        }
      }).then(response => {
        console.log("Response")
        console.log(response)
      }).catch(err => {
        console.log("Error")
        console.error(err)
      })
    }
  }
}
</script>
