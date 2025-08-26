// document.addEventListener('DOMContentLoaded', function() {
//     const loginForm = document.getElementById('loginForm');
//     const messageDiv = document.getElementById('message');

//     loginForm.addEventListener('submit', function(e) {
//         e.preventDefault();
        
//         const username = document.getElementById('username').value;
//         const password = document.getElementById('password').value;
        
//         // 简单的表单验证
//         if (!username || !password) {
//             showMessage('请填写用户名和密码', 'error');
//             return;
//         }
        
//         if (username.length < 3) {
//             showMessage('用户名至少需要3个字符', 'error');
//             return;
//         }
        
//         if (password.length < 6) {
//             showMessage('密码至少需要6个字符', 'error');
//             return;
//         }
        
//         // 模拟登录请求
//         simulateLogin(username, password);
//     });

//     function simulateLogin(username, password) {
//         // 显示加载状态
//         showMessage('正在登录...', 'info');
        
//         // 模拟网络请求延迟
//         setTimeout(() => {
//             // 这里是模拟的登录逻辑
//             // 实际应用中应该发送请求到后端API
//             if (username === 'admin' && password === '123456') {
//                 showMessage('登录成功！正在跳转...', 'success');
//                 // 模拟跳转延迟
//                 setTimeout(() => {
//                     alert('欢迎 ' + username + '！');
//                     // 实际应用中这里应该跳转到主页
//                     // window.location.href = '/dashboard';
//                 }, 1500);
//             } else {
//                 showMessage('用户名或密码错误', 'error');
//             }
//         }, 1000);
//     }

//     function showMessage(text, type) {
//         messageDiv.textContent = text;
//         messageDiv.className = 'message ' + type;
        
//         // 3秒后自动清除消息（除了成功消息）
//         if (type !== 'success') {
//             setTimeout(() => {
//                 messageDiv.textContent = '';
//                 messageDiv.className = 'message';
//             }, 3000);
//         }
//     }
// });

document.getElementById('loginForm').addEventListener('submit', function(e) {
    e.preventDefault();
    
    const username = document.getElementById('username').value;
    const password = document.getElementById('password').value;
    
    if (username && password) {
        alert('登录功能需要后端支持，当前仅为前端演示');
        // 这里可以添加AJAX请求到后端进行验证
    } else {
        alert('请填写完整的登录信息');
    }
});