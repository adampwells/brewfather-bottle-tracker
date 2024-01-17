FROM nginx
RUN mkdir /brewfather-bottle-tracker
COPY web/dist /brewfather-bottle-tracker
COPY nginx.conf /etc/nginx/nginx.conf
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
