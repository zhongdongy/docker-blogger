# Eastwind Blogger

一个使用全缓存实现的相当简单却高效的静态博客服务器程序。


## 配置和部署

### 示例配置

```yaml
public:
  home_post: welcome
site:
  hostname: "127.0.0.1:5000"
  enable_https: false
  site_name: 那阵东风
  site_email: zhongdongy@dongs.xyz
  site_slogan: '由 Eastwind Blogger 驱动'
  beian:
    enabled: true
    icp_id: 粤ICP备XXX号-1
    beian_id: 粤公安网备XXX号
```

## 撰写文章

您可以使用 Markdown 和 LaTeX（使用 MathJax 进行渲染）撰写博文。为了提供标签分类等丰富的博文索引服务，您还可以在每篇博文的报头增加一段元数据：

```yaml
---
title: 沁园春·雪
author: 毛泽东
author_email: "mzd@example.com"
created_at: "1936-02-01"
updated_at: "1936-02-01"
tags:
- 毛泽东诗词
permanent_link: 
renderer_params: 
- content-serif
- disable-toc
---
```

这段元数据遵循 YAML 语法，其中：
- `title` 对应了文章的标题
- `author` 指博文的作者
- `author_email` 指博文作者的邮箱，用于获取 GRAVATAR 图像
- `created_at` 指博文创作的时间，您可以使用“YYYY-mm-dd”，也可以精确到分秒“YYYY-mm-dd HH:MM:SS”
- `updated_at` 指博文修改的时间，您可以使用“YYYY-mm-dd”，也可以精确到分秒“YYYY-mm-dd HH:MM:SS”
- `tags` 指博文的标签，您可以为同一篇博文提供任意数量的标签
- `permanent_link` 是您手动指定的博文标题
- `renderer_params` 是渲染博文时的指令参数，您可以使用：
  - `disable-toc`：关闭文章索引目录 TOC
  - `content-serif`：使用衬线字体 (Noto Serif SC) 渲染博文内容

为了丰富读者的阅读体验，您可以使用 Markdown 扩展语法中的脚注来实现。