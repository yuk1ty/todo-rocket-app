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
          <el-button type="primary">Submit</el-button>
        </el-form-item>
      </el-form>
    </el-main>
  </el-container>
</template>

<script>
export default {
  data() {
    return {
      description: '',
      due: '',
    }
  }
}
</script>
